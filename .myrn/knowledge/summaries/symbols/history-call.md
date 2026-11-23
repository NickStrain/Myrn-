---
id: history-call
type: summary
status: unverified
title: history::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/history.rs#tools::history::call
anchors:
- sym:crates/myrn-mcp/src/tools/history.rs#tools::history::call@2cd0fe3bb5fbbcb5
map_version: '2'
---
# history::call

Reads path and optional limit from args, runs engine.history, and returns the rendered commit text.

## Behaviour

- args without path -> the error from str_arg is returned
- no limit, or a value that is not a non-negative integer (negative, float, string) -> limit is 15
- limit 0 is passed to engine.history unchanged; no special case here

## Terms

history lookup, commit history query, git log lookup, path query, result limit, argument parsing

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs:20-24`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-history-rs]]
- **Calls:** [[engine-history]], [[history-render]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
