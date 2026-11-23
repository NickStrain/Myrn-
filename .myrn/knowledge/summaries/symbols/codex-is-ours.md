---
id: codex-is-ours
type: summary
status: unverified
title: codex::is_ours
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::is_ours
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::is_ours@313e2c33620b554d
map_version: '2'
---
# codex::is_ours

Returns true when a hook entry's command field contains the ' hook codex ' marker, which identifies hooks myrn installed.

## Behaviour

- command ending in 'myrn' hook codex Stop' -> true
- no command field, or a non-string value -> false
- a hook from another tool without the marker -> false

## Terms

ownership check, managed hook test, myrn hook match

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:50-54`
- **Signature:** `fn is_ours(h: &Value) -> bool`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
