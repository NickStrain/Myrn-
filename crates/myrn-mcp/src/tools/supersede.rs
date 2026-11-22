//! supersede(id, item): replace a record that no longer holds with a corrected one.

use anyhow::Result;
use myrn_engine::api::Engine;
use serde_json::{json, Value};

use super::{schema, str_arg};

pub fn spec() -> Value {
    json!({
        "name": "supersede",
        "description": "Replace a record that no longer holds (often one shown as possibly outdated) with a \
            corrected one. `item` takes the same fields as `record` (type, body, about, evidence, ...). The old \
            record is kept in history, marked superseded, and is no longer shown.",
        "inputSchema": schema(json!({
            "id": { "type": "string", "description": "Id of the record being replaced." },
            "item": { "type": "object", "description": "The corrected record, with the fields of `record`." }
        }), &["id", "item"]),
    })
}

pub fn call(engine: &mut Engine, args: &Value) -> Result<String> {
    let id = str_arg(args, "id")?;
    let item = args
        .get("item")
        .filter(|v| v.is_object())
        .ok_or_else(|| anyhow::anyhow!("missing required argument `item` (an object)"))?;
    Ok(engine.supersede(id, item)?.render())
}
