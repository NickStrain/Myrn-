---
id: adapter-read-json
type: summary
status: unverified
title: adapter::read_json
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::read_json
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::read_json@6e2d43a4718cb873
---
# adapter::read_json

Reads a JSON file into a Value. A missing or blank file yields an empty object; invalid JSON returns an error that names the path.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:112-120`
- **Signature:** `pub(crate) fn read_json(path: &Path) -> Result<Value>`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]], [[codex-install]], [[codex-uninstall]], [[codex-inspect]]
- **Tests:** `install_is_idempotent_and_keeps_other_hooks`, `install_preserves_config_and_uninstalls_cleanly`
