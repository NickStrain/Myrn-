//! explain(symbol): what it does, who calls it, what is known about it.

use anyhow::Result;
use myrn_engine::api::Engine;
use serde_json::{json, Value};

use super::{schema, str_arg};

pub fn spec() -> Value {
    json!({
        "name": "explain",
        "description": "Card for one symbol or file: signature, summary, location, what it calls, who calls it, \
            which tests cover it and recent commits. Accepts a name, qualified name (a::b or A.b), path, or path:line.",
        "inputSchema": schema(json!({
            "symbol": { "type": "string", "description": "Symbol name, qualified name, file path or path:line." }
        }), &["symbol"]),
    })
}

pub fn call(engine: &mut Engine, args: &Value) -> Result<String> {
    let q = str_arg(args, "symbol")?;
    Ok(engine.explain(q)?.render(|e| e.render()))
}
