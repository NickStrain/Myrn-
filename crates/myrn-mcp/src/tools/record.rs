//! record(item): store a decision, bug, workflow, fact or symbol summary after validation.

use anyhow::Result;
use myrn_engine::api::Engine;
use serde_json::{json, Value};

use super::schema;

pub fn spec() -> Value {
    let names = json!({
        "oneOf": [{ "type": "string" }, { "type": "array", "items": { "type": "string" } }]
    });
    json!({
        "name": "record",
        "description": "Store one thing this session learned, so later sessions don't rediscover it: a bug with its \
            root cause and fix, a decision and why, a workflow that worked, a non-obvious fact, or a summary of a \
            symbol (what it does, inputs, outputs, edge cases, gotchas). Put several items in `items` to store them \
            in one call. Each item is checked: evidence must cite an existing symbol, commit, test or test command \
            you ran; a repeat of a known record updates it; a conflicting one is flagged. Rejections say what to fix.",
        "inputSchema": schema(json!({
            "type": { "type": "string", "enum": ["bug", "decision", "workflow", "fact", "summary"] },
            "title": { "type": "string", "description": "Short title (optional)." },
            "body": { "type": "string", "description": "What was learned, in plain sentences. For a summary: what the symbol does, inputs, outputs, edge cases, gotchas." },
            "about": names.clone(),
            "evidence": {
                "type": "array",
                "description": "At least one: {\"symbol\": name}, {\"commit\": sha}, {\"test\": test name or file}, {\"test_run\": command you ran}.",
                "items": { "type": "object", "additionalProperties": { "type": "string" } }
            },
            "error": { "type": "string", "description": "Bug: the error text as seen." },
            "root_cause": { "type": "string", "description": "Bug: why it happened." },
            "located_in": names.clone(),
            "fix": { "type": "string", "description": "Bug: how it was fixed." },
            "fixed_in": { "type": "string", "description": "Bug: the fixing commit." },
            "steps": { "type": "array", "items": { "type": "string" }, "description": "Workflow: ordered steps." },
            "verified_by": names,
            "relations": {
                "type": "array",
                "description": "Typed links to other records: rel is one of supersedes, contradicts, related, avoids, realized_in, implemented_by, fixes.",
                "items": { "type": "object", "properties": { "rel": { "type": "string" }, "target": { "type": "string" } }, "required": ["rel", "target"] }
            },
            "session": { "type": "string", "description": "Session id (optional)." },
            "items": {
                "type": "array",
                "description": "Several records at once, each with the fields above. Use this instead of the top-level fields.",
                "items": { "type": "object" }
            }
        }), &[]),
    })
}

pub fn call(engine: &mut Engine, args: &Value) -> Result<String> {
    let outcomes = engine.record_any_json(args)?;
    if outcomes.len() == 1 {
        return Ok(outcomes[0].render());
    }
    let ok = outcomes.iter().filter(|o| o.accepted).count();
    let mut out = format!("{ok} of {} items stored.\n", outcomes.len());
    for (i, o) in outcomes.iter().enumerate() {
        out.push_str(&format!("\nItem {}: {}", i + 1, o.render()));
    }
    Ok(out)
}
