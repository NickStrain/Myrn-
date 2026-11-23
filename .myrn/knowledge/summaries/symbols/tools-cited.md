---
id: tools-cited
type: summary
status: unverified
title: tools::cited
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/mod.rs#tools::cited
anchors:
- sym:crates/myrn-mcp/src/tools/mod.rs#tools::cited@465c90962c33d539
map_version: '2'
---
# tools::cited

Extracts the node ids a tool call refers to, based on the tool name and its arguments. Tools without a rule return an empty list.

## Behaviour

- explain or impact with symbol that matches -> [id of the first hit]; no match, error, or no symbol arg -> []
- confirm or supersede with id "n1" -> ["n1"]; missing id -> []
- record with items [{relations:[{target:"a"},{target:"b"}]}] -> ["a","b"]; without an items array it reads relations from the top-level args

## Terms

cited ids, referenced nodes, used ids, relation targets, referenced symbol, first hit, injected context

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs:65-100`
- **Signature:** `fn cited(engine: &Engine, name: &str, args: &Value) -> Vec<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Calls:** [[engine-find]]
- **Called by:** [[tools-call]]
