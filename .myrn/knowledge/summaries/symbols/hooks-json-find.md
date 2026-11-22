---
id: hooks-json-find
type: summary
status: unverified
title: hooks_json::find
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::find
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::find@e8ad55a23606055b
---
# hooks_json::find

Read-only. Returns references to handlers under hooks[event] across all groups that is_ours matches.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:186-200`
- **Signature:** `pub fn find<'a>( doc: &'a Value, event: &str, is_ours: &dyn Fn(&Value) -> bool, ) -> Vec<&'a Value>`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-inspect]], [[codex-inspect]]
- **Tests:** `install_is_idempotent_and_keeps_other_hooks`, `install_preserves_config_and_uninstalls_cleanly`
