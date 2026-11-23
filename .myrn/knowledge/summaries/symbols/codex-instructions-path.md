---
id: codex-instructions-path
type: summary
status: unverified
title: codex::instructions_path
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::instructions_path
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::instructions_path@458a1e33b3e74412
map_version: '2'
---
# codex::instructions_path

Returns <root>/AGENTS.md, the file that receives the pull-only instruction block. It does no I/O.

## Behaviour

- /repo -> /repo/AGENTS.md
- file need not exist

## Terms

agents file, instruction file, pull only file

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:42-44`
- **Signature:** `fn instructions_path(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-install]], [[codex-uninstall]], [[codex-inspect]]
