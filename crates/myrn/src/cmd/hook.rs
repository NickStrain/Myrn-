//! myrn hook <agent> <event>: called by agent hooks. Reads the payload on
//! stdin, translates it through the adapter, and answers on stdout.
//!
//! A hook must never get in the agent's way: every failure is swallowed and the
//! process exits 0 with nothing on stdout. Projects that are not set up, or not
//! connected for this agent, are ignored.

use std::io::Read;
use std::process::ExitCode;
use std::time::Instant;

use anyhow::Result;
use myrn_engine::api::Engine;
use myrn_model::event::EventKind;
use myrn_store::state::HookTiming;

use super::Ctx;

const MAX_PAYLOAD: u64 = 4 * 1024 * 1024;

pub fn run(ctx: &Ctx, agent: &str, event: &str) -> Result<ExitCode> {
    let started = Instant::now();
    match handle(ctx, agent, event, started) {
        Ok(out) => {
            if !out.is_empty() {
                println!("{out}");
            }
        }
        Err(e) => eprintln!("myrn hook: {e:#}"),
    }
    Ok(ExitCode::SUCCESS)
}

fn handle(ctx: &Ctx, agent: &str, event_arg: &str, started: Instant) -> Result<String> {
    let mut raw = String::new();
    std::io::stdin()
        .take(MAX_PAYLOAD)
        .read_to_string(&mut raw)?;
    let payload: serde_json::Value = serde_json::from_str(&raw).unwrap_or(serde_json::Value::Null);
    let Some(adapter) = myrn_adapters::by_name(agent) else {
        return Ok(String::new());
    };
    let Some(event) = adapter.translate(event_arg, &payload)? else {
        return Ok(String::new());
    };

    let dir = event
        .cwd
        .clone()
        .map(std::path::PathBuf::from)
        .filter(|p| p.is_dir())
        .or_else(|| std::env::var_os("CLAUDE_PROJECT_DIR").map(Into::into))
        .unwrap_or(std::env::current_dir()?);
    let Ok(mut engine) = Engine::open(&dir, ctx.data_dir.as_deref()) else {
        return Ok(String::new());
    };
    if !engine.state()?.agents.contains_key(agent) {
        return Ok(String::new());
    }
    let inject = engine.config.context.inject;

    let context = match &event.kind {
        EventKind::SessionStart { .. } => {
            engine.refresh()?;
            if inject {
                engine.brief(None)?.text
            } else {
                String::new()
            }
        }
        EventKind::PromptSubmitted { prompt } if inject => {
            engine.refresh()?;
            let pack = engine.context(prompt, None)?;
            if !pack.text.is_empty() {
                engine.log_injection(&event.session_id, &pack.items)?;
            }
            pack.text
        }
        // Tool results and session ends are recorded by session capture (next step).
        _ => String::new(),
    };
    let out = adapter.format(event_arg, &context);

    let mut state = engine.state()?;
    state.record_timing(HookTiming {
        agent: agent.into(),
        event: event_arg.into(),
        millis: started.elapsed().as_millis() as u64,
        time: super::connect::now(),
    });
    engine.save_state(&state)?;
    Ok(out)
}
