---
id: codex-codex-dir
type: summary
status: unverified
title: codex::codex_dir
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::codex_dir
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::codex_dir@2df6305824666d85
---
# codex::codex_dir

Returns root/.codex, the directory that holds Codex's hooks and config files.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:33-35`
- **Signature:** `fn codex_dir(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-hooks-path]], [[codex-config-path]], [[codex-write-mcp]], [[codex-uninstall]]
