---
id: mode-as-str
type: summary
status: unverified
title: Mode::as_str
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Mode::as_str
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Mode::as_str@b44ce0b53965fa52
map_version: '2'
---
# Mode::as_str

Returns the stored string id for a Mode, either hooks or pull_only.

## Behaviour

- Hooks -> 'hooks'
- PullOnly -> 'pull_only'

## Terms

mode name, serialized mode, mode string

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:28-33`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[connect-run]]
