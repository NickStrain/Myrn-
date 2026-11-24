//! myrn record: store what a session learned, from JSON on stdin. The same
//! input and checks as the MCP `record` tool, for agents connected without
//! the MCP server (the default with hooks).

use std::io::Read;
use std::process::ExitCode;

use anyhow::{Context, Result};

use super::Ctx;

pub fn run(ctx: &Ctx) -> Result<ExitCode> {
    let mut text = String::new();
    std::io::stdin()
        .read_to_string(&mut text)
        .context("reading the record JSON from stdin")?;
    let v: serde_json::Value = serde_json::from_str(text.trim())
        .context("stdin is not JSON: give one item, or {\"items\": [...]} with several")?;
    let mut engine = ctx.engine()?;
    let outcomes = engine.record_any_json(&v)?;
    let ok = outcomes.iter().filter(|o| o.accepted).count();
    let code = if ok == outcomes.len() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    };
    let _ = ctx.emit(&outcomes, || {
        if outcomes.len() == 1 {
            return outcomes[0].render();
        }
        let mut out = format!("{ok} of {} items stored.\n", outcomes.len());
        for (i, o) in outcomes.iter().enumerate() {
            out.push_str(&format!("\nItem {}: {}", i + 1, o.render()));
        }
        out
    })?;
    Ok(code)
}
