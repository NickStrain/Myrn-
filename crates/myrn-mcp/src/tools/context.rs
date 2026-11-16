//! context(task): a ranked context pack for a described task.

use anyhow::Result;
use myrn_engine::api::Engine;
use serde_json::{json, Value};

use super::{schema, str_arg};

pub fn spec() -> Value {
    json!({
        "name": "context",
        "description": "Ranked context for a task: the relevant symbols with locations, their callers, callees and \
            tests, within a token budget. Call this first, before searching or opening files.",
        "inputSchema": schema(json!({
            "task": { "type": "string", "description": "What you are about to do, in your own words. Include names, paths or errors you know." },
            "budget": { "type": "integer", "description": "Token budget (default 1500)." }
        }), &["task"]),
    })
}

pub fn call(engine: &mut Engine, args: &Value) -> Result<String> {
    let task = str_arg(args, "task")?;
    let budget = args
        .get("budget")
        .and_then(Value::as_u64)
        .map(|b| b as usize);
    let pack = engine.context(task, budget)?;
    if pack.text.is_empty() {
        return Ok("Nothing in the code graph is clearly relevant to this task.".into());
    }
    engine.log_injection("mcp", &pack.items)?;
    Ok(pack.text)
}
