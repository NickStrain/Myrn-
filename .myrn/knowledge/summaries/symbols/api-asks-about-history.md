---
id: api-asks-about-history
type: summary
status: unverified
title: api::asks_about_history
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::asks_about_history
anchors:
- sym:crates/myrn-engine/src/api.rs#api::asks_about_history@d16a47875af27ce9
---
# api::asks_about_history

Returns true when the task contains history words such as commit, regression, changed, reverted, blame or 'used to', which allows commits into context packs.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1260-1285`
- **Signature:** `fn asks_about_history(task: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-context-excluding]]
