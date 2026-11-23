---
id: tools-list
type: summary
status: unverified
title: tools::list
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/mod.rs#tools::list
anchors:
- sym:crates/myrn-mcp/src/tools/mod.rs#tools::list@d158b962f078f186
map_version: '2'
---
# tools::list

Returns the JSON specs for all eight tools in a fixed order. Each entry comes from that tool module's `spec()`.

## Behaviour

- no arguments -> Vec of 8 spec values, ordered context, explain, impact, history, recall, record, confirm, supersede
- does not check the engine or filter anything; the list is always the same

## Terms

tool list, tool specs, advertise tools, tool catalogue, tool manifest

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs:17-28`
- **Signature:** `pub fn list() -> Vec<Value>`
- **In:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Calls:** [[confirm-spec]], [[context-spec]], [[explain-spec]], [[history-spec]], [[impact-spec]], [[recall-spec]], [[record-spec]], [[supersede-spec]]
- **Called by:** [[server-handle]]
