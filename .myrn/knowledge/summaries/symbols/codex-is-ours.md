---
id: codex-is-ours
type: summary
status: unverified
title: codex::is_ours
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::is_ours
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::is_ours@313e2c33620b554d
---
# codex::is_ours

Returns true when a hook entry's command contains ' hook codex ', so myrn can find and remove its own entries without touching others.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:50-54`
- **Signature:** `fn is_ours(h: &Value) -> bool`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
