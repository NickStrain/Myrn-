---
id: engine-links
type: summary
status: unverified
title: Engine::links
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::links
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::links@9df787c1689f8196
map_version: '2'
---
# Engine::links

Converts node and status pairs into Link values.

## Behaviour

- Empty input -> empty list
- Status None stays None on the link

## Terms

link list, neighbours, mapping

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:396-398`
- **Signature:** `fn links(v: Vec<(Node, Option<String>)>) -> Vec<Link>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[link-new]]
- **Called by:** [[engine-symbol-card]], [[engine-file-card]], [[engine-impact]]
