//! recall(error): past bugs matching an error, with causes and fixes.

use anyhow::Result;
use myrn_engine::api::Engine;
use serde_json::{json, Value};

use super::{schema, str_arg};

pub fn spec() -> Value {
    json!({
        "name": "recall",
        "description": "Known bugs matching an error message or failing test output, with their root cause, the code \
            responsible and the fix, from earlier sessions. Paste the error as you see it.",
        "inputSchema": schema(json!({
            "error": { "type": "string", "description": "The error text or failing test output." },
            "budget": { "type": "integer", "description": "Token budget (default 1500)." }
        }), &["error"]),
    })
}

pub fn call(engine: &mut Engine, args: &Value) -> Result<String> {
    let error = str_arg(args, "error")?;
    let budget = args.get("budget").and_then(Value::as_u64).unwrap_or(1500) as usize;
    let (text, ids) = engine.recall_text(error, budget)?;
    if text.is_empty() {
        return Ok("No recorded bug matches this error.".into());
    }
    engine.log_injection("mcp", &ids)?;
    Ok(text)
}
