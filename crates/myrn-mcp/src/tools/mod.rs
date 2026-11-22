//! One module per MCP tool.

pub mod confirm;
pub mod context;
pub mod explain;
pub mod history;
pub mod impact;
pub mod recall;
pub mod record;
pub mod resolve;
pub mod supersede;

use anyhow::Result;
use myrn_engine::api::Engine;
use serde_json::{json, Value};

pub fn list() -> Vec<Value> {
    vec![
        context::spec(),
        explain::spec(),
        impact::spec(),
        history::spec(),
        recall::spec(),
        record::spec(),
        confirm::spec(),
        supersede::spec(),
    ]
}

/// Run a tool; `None` if no tool has that name.
pub fn call(engine: &mut Engine, name: &str, args: &Value) -> Option<Result<String>> {
    if ![
        "context",
        "explain",
        "impact",
        "history",
        "recall",
        "record",
        "confirm",
        "supersede",
    ]
    .contains(&name)
    {
        return None;
    }
    // Keep the index current without re-walking the tree on every call in a burst.
    if let Err(e) = engine.refresh_if_older(2) {
        return Some(Err(e));
    }
    Some(match name {
        "context" => context::call(engine, args),
        "explain" => explain::call(engine, args),
        "impact" => impact::call(engine, args),
        "recall" => recall::call(engine, args),
        "record" => record::call(engine, args),
        "confirm" => confirm::call(engine, args),
        "supersede" => supersede::call(engine, args),
        _ => history::call(engine, args),
    })
}

pub(crate) fn str_arg<'a>(args: &'a Value, key: &str) -> Result<&'a str> {
    args.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("missing required argument `{key}`"))
}

pub(crate) fn schema(props: Value, required: &[&str]) -> Value {
    json!({ "type": "object", "properties": props, "required": required })
}
