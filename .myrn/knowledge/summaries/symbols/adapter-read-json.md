---
id: adapter-read-json
type: summary
status: unverified
title: adapter::read_json
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::read_json
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::read_json@6e2d43a4718cb873
map_version: '2'
---
# adapter::read_json

Reads a JSON object file and returns an empty object when the file is missing or blank.

## Behaviour

- Missing file -> {}
- Blank or whitespace-only file -> {}
- Invalid JSON -> Err naming the file path

## Terms

load json, config file, parse json, empty object, missing file

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:112-120`
- **Signature:** `pub(crate) fn read_json(path: &Path) -> Result<Value>`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]], [[codex-install]], [[codex-uninstall]], [[codex-inspect]]
- **Tests:** `install_is_idempotent_and_keeps_other_hooks`, `install_preserves_config_and_uninstalls_cleanly`
