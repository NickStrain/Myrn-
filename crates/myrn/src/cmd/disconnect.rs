//! myrn disconnect: remove hooks, MCP registration and instruction lines.

use std::process::ExitCode;

use anyhow::Result;

use super::{connect::install_ctx, Ctx};

pub fn run(ctx: &Ctx, agents: Vec<String>) -> Result<ExitCode> {
    let cwd = std::env::current_dir()?;
    let engine = myrn_engine::api::Engine::open(&cwd, ctx.data_dir.as_deref())?;
    let mut state = engine.state()?;
    let targets: Vec<String> = if agents.is_empty() {
        state.agents.keys().cloned().collect()
    } else {
        agents
    };
    if targets.is_empty() {
        println!("No agents are connected.");
        return Ok(ExitCode::SUCCESS);
    }
    let ictx = install_ctx(engine.root())?;
    for name in targets {
        let Some(a) = myrn_adapters::by_name(&name) else {
            println!("unknown agent `{name}`");
            continue;
        };
        let created = state
            .agents
            .get(&name)
            .map(|s| s.created.clone())
            .unwrap_or_default();
        let changes = a.uninstall(&ictx, &created)?;
        println!("{}:", a.display_name());
        if changes.is_empty() {
            println!("  nothing to remove");
        }
        for c in changes {
            println!("  {:<34} {}", c.path, c.what);
        }
        state.agents.remove(&name);
    }
    if !state.agents.values().any(|s| s.mode == "hooks") {
        state.capture_consent = false;
        state.consent_time = None;
    }
    engine.save_state(&state)?;
    Ok(ExitCode::SUCCESS)
}
