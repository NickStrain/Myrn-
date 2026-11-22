---
id: hooks-json-remove
type: summary
status: unverified
title: hooks_json::remove
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::remove
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::remove@a49c5e5dccf6ef59
---
# hooks_json::remove

Deletes every handler that is_ours matches, then drops groups and event keys left empty and removes the hooks key when nothing remains. Returns the number of handlers removed.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:135-164`
- **Signature:** `pub fn remove(doc: &mut Value, is_ours: &dyn Fn(&Value) -> bool) -> usize`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[codex-install]], [[codex-uninstall]]
