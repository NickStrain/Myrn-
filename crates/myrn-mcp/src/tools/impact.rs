//! impact(symbol): dependents and covering tests.

use anyhow::Result;
use myrn_engine::api::Engine;
use serde_json::{json, Value};

use super::{schema, str_arg};

pub fn spec() -> Value {
    json!({
        "name": "impact",
        "description": "What depends on a symbol or file: direct and indirect callers (up to 3 hops), importing \
            files, and the tests that reach it. Use before changing a signature or behaviour.",
        "inputSchema": schema(json!({
            "symbol": { "type": "string", "description": "Symbol name, qualified name, file path or path:line." }
        }), &["symbol"]),
    })
}

pub fn call(engine: &mut Engine, args: &Value) -> Result<String> {
    let q = str_arg(args, "symbol")?;
    Ok(engine.impact(q)?.render(|r| r.render(15)))
}
