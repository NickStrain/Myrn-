//! confirm(id): a record marked possibly outdated still holds for the current code.

use anyhow::Result;
use myrn_engine::api::Engine;
use serde_json::{json, Value};

use super::{schema, str_arg};

pub fn spec() -> Value {
    json!({
        "name": "confirm",
        "description": "Confirm that a record shown as possibly outdated (its code changed since it was written) \
            still holds for the code as it is now. Its anchors move to the current code and it returns to its \
            previous state. Check the code first; if the record no longer holds, use `supersede` instead.",
        "inputSchema": schema(json!({
            "id": { "type": "string", "description": "Record id, e.g. bug-7f3a91." }
        }), &["id"]),
    })
}

pub fn call(engine: &mut Engine, args: &Value) -> Result<String> {
    let id = str_arg(args, "id")?;
    let c = engine.confirm(id)?;
    Ok(format!(
        "{} confirmed for the current code ({} anchor{} refreshed); status: {}.",
        c.id,
        c.anchors_refreshed,
        if c.anchors_refreshed == 1 { "" } else { "s" },
        c.status
    ))
}
