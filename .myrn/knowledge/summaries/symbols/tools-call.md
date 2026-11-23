---
id: tools-call
type: summary
status: unverified
title: tools::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/mod.rs#tools::call
anchors:
- sym:crates/myrn-mcp/src/tools/mod.rs#tools::call@5d4452202fa267d3
map_version: '2'
---
# tools::call

Routes a tool name to its handler, returning None for unknown names. For known names it first refreshes the index if it is older than 2 (per `refresh_if_older`) and marks the cited node ids as used, ignoring any error from marking.

## Behaviour

- name not in the eight-name list (e.g. "nope") -> None, with no refresh and no marking
- refresh_if_older fails -> Some(Err(e)) and the handler does not run
- known name -> cited ids are marked used, then Some(handler result); a `_` fallback arm would route to history but is unreachable after the name check

## Terms

dispatch, route, tool router, unknown tool, mark used, index refresh, stale index, burst of calls

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs:31-62`
- **Signature:** `pub fn call(engine: &mut Engine, name: &str, args: &Value) -> Option<Result<String>>`
- **In:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Calls:** [[engine-refresh-if-older]], [[confirm-call]], [[context-call]], [[explain-call]], [[history-call]], [[impact-call]], [[tools-cited]], [[recall-call]], [[record-call]], [[supersede-call]]
- **Called by:** [[server-handle]]
