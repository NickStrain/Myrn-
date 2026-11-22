---
id: history-call
type: summary
status: unverified
title: history::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/history.rs#tools::history::call
anchors:
- sym:crates/myrn-mcp/src/tools/history.rs#tools::history::call@2cd0fe3bb5fbbcb5
---
# history::call

Reads the required string `path` (errors if missing or not a string) and an optional `limit` (defaults to 15 when absent or not a u64), calls `engine.history(path, limit)`, and returns the rendered text. Engine errors propagate via `?`.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs:20-24`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-history-rs]]
- **Calls:** [[engine-history]], [[history-render]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
