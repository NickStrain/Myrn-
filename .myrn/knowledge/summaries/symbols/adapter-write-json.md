---
id: adapter-write-json
type: summary
status: unverified
title: adapter::write_json
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::write_json
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::write_json@d09d8394040f25b8
---
# adapter::write_json

Writes a value as pretty-printed JSON with a trailing newline, creating parent directories first and overwriting any existing file.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:122-128`
- **Signature:** `pub(crate) fn write_json(path: &Path, v: &Value) -> Result<()>`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[codex-install]], [[codex-uninstall]]
- **Tests:** `install_is_idempotent_and_keeps_other_hooks`
