---
id: hooks-json-remove
type: summary
status: unverified
title: hooks_json::remove
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::remove
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::remove@a49c5e5dccf6ef59
map_version: '2'
---
# hooks_json::remove

Deletes every handler matching is_ours under doc.hooks and returns how many were removed. Groups, events and the hooks key itself are dropped when they end up empty.

## Behaviour

- No hooks key -> returns 0 and doc is unchanged
- Group whose handlers all match -> group removed
- Group with no hooks array -> kept as is

## Terms

delete hook, strip handlers, clean up, remove our hooks, prune empty

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:135-164`
- **Signature:** `pub fn remove(doc: &mut Value, is_ours: &dyn Fn(&Value) -> bool) -> usize`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[codex-install]], [[codex-uninstall]]
