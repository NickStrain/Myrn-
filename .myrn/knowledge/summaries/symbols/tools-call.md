---
id: tools-call
type: summary
status: unverified
title: tools::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/mod.rs#tools::call
anchors:
- sym:crates/myrn-mcp/src/tools/mod.rs#tools::call@46d40731ef230240
---
# tools::call

Returns None unless name is one of the six exposed tools. Otherwise it first calls engine.refresh_if_older(2), returning that error as Some(Err) and skipping dispatch on failure, then routes to the matching submodule's call with args. Any name not matched explicitly falls through to history::call.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs:28-48`
- **Signature:** `pub fn call(engine: &mut Engine, name: &str, args: &Value) -> Option<Result<String>>`
- **In:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Calls:** [[engine-refresh-if-older]], [[context-call]], [[explain-call]], [[history-call]], [[impact-call]], [[recall-call]], [[record-call]]
- **Called by:** [[server-handle]]
