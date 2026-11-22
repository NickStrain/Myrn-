---
id: tools-list
type: summary
status: unverified
title: tools::list
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/mod.rs#tools::list
anchors:
- sym:crates/myrn-mcp/src/tools/mod.rs#tools::list@28b382255f8ac912
---
# tools::list

Returns a Vec of the six tool spec JSON values from context, explain, impact, history, recall, and record. Excludes resolve. Takes no arguments and has no side effects.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs:16-25`
- **Signature:** `pub fn list() -> Vec<Value>`
- **In:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Calls:** [[context-spec]], [[explain-spec]], [[history-spec]], [[impact-spec]], [[recall-spec]], [[record-spec]]
- **Called by:** [[server-handle]]
