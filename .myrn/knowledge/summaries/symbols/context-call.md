---
id: context-call
type: summary
status: unverified
title: context::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/context.rs#tools::context::call
anchors:
- sym:crates/myrn-mcp/src/tools/context.rs#tools::context::call@aa0358c1c23f370a
---
# context::call

Reads `task` (required, errors if missing or not a string) and optional `budget` from args, then calls `engine.context`. Non-integer or negative `budget` values are silently treated as absent. If the pack text is empty it returns a fixed no-results message without logging. Otherwise it calls `engine.log_injection("mcp", ...)` (side effect, error propagated) and returns the pack text.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/context.rs:21-33`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-context-rs]]
- **Calls:** [[engine-context]], [[engine-log-injection]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
