---
id: engine-links
type: summary
status: unverified
title: Engine::links
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::links
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::links@9df787c1689f8196
---
# Engine::links

Converts (node, optional status) pairs into Link values.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:371-373`
- **Signature:** `fn links(v: Vec<(Node, Option<String>)>) -> Vec<Link>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[link-new]]
- **Called by:** [[engine-symbol-card]], [[engine-file-card]], [[engine-impact]]
