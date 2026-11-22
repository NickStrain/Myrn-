//! myrn hook <agent> <event>: called by agent hooks. Reads the payload on
//! stdin, translates it through the adapter, and answers on stdout.
//!
//! A hook must never get in the agent's way: every failure is swallowed and the
//! process exits 0 with nothing on stdout. Projects that are not set up, or not
//! connected for this agent, are ignored. With consent, every event is also
//! appended (redacted) to the session's episode log.

use std::io::Read;
use std::process::ExitCode;
use std::time::Instant;

use anyhow::Result;
use myrn_engine::api::Engine;
use myrn_model::event::{EventKind, ToolOutcome};
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
                let pack = engine.brief(None)?;
                if !pack.text.is_empty() {
                    engine.log_injection(&event.session_id, &pack.items)?;
                }
                pack.text
            } else {
                String::new()
            }
        }
        EventKind::PromptSubmitted { prompt } if inject => {
            engine.refresh()?;
            let seen = engine.injected_in_session(&event.session_id)?;
            let pack = engine.context_excluding(prompt, None, &seen)?;
            if !pack.text.is_empty() {
                engine.log_injection(&event.session_id, &pack.items)?;
            }
            pack.text
        }
        // A failing tool: if the error matches a known bug, hand over its cause and fix.
        EventKind::ToolResult(r)
            if r.outcome == ToolOutcome::Failure && engine.config.records.recall_on_failure =>
        {
            match r.error.as_deref().or(r.command.as_deref()) {
                Some(err) => {
                    let (text, ids) =
                        engine.recall_text(err, engine.config.records.failure_budget)?;
                    if !ids.is_empty() {
                        engine.log_injection(&event.session_id, &ids)?;
                    }
                    text
                }
                None => String::new(),
            }
        }
        // Working in a file with records written before its code changed: show
        // them once per session and ask the agent to confirm or supersede them.
        EventKind::ToolResult(r) if inject && !r.files.is_empty() => {
            let root = engine.root().to_path_buf();
            let files: Vec<String> = r
                .files
                .iter()
                .filter_map(|f| {
                    let p = std::path::Path::new(f);
                    let abs = if p.is_absolute() {
                        p.to_path_buf()
                    } else {
                        root.join(p)
                    };
                    myrn_store::paths::repo_rel(&root, &abs)
                })
                .collect();
            let seen = engine.injected_in_session(&event.session_id)?;
            let (text, ids) =
                engine.suspect_notice(&files, &seen, engine.config.records.failure_budget)?;
            if !ids.is_empty() {
                engine.log_injection(&event.session_id, &ids)?;
            }
            text
        }
        _ => String::new(),
    };
    // Recording must never cost the agent its context: report and carry on.
    if let Err(e) = engine.capture(agent, &event) {
        eprintln!("myrn hook: recording failed: {e:#}");
    }
    // Evidence may have come in (a test passed, a fix landed): promote records.
    if matches!(event.kind, EventKind::SessionEnd { .. }) {
        if let Err(e) = engine.promote() {
            eprintln!("myrn hook: verifying records failed: {e:#}");
        }
    }
    // End of a turn that edited code: ask the agent to record what it learned.
    let context = match &event.kind {
        EventKind::SessionEnd { reason, continued } if reason.as_deref() == Some("stop") => {
            match engine.records_request(&event.session_id, *continued) {
                Ok(Some(ask)) => ask,
                Ok(None) => context,
                Err(e) => {
                    eprintln!("myrn hook: record request failed: {e:#}");
                    context
                }
            }
        }
        _ => context,
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
