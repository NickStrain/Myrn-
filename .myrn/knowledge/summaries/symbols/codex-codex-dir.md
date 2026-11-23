---
id: codex-codex-dir
type: summary
status: unverified
title: codex::codex_dir
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::codex_dir
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::codex_dir@2df6305824666d85
map_version: '2'
---
# codex::codex_dir

Returns the .codex directory under the repo root. It does no I/O.

## Behaviour

- /repo -> /repo/.codex
- does not check that the directory exists

## Terms

dot codex folder, project config folder, codex directory path

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:33-35`
- **Signature:** `fn codex_dir(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-hooks-path]], [[codex-config-path]], [[codex-write-mcp]], [[codex-uninstall]]
