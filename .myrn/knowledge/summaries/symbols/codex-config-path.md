---
id: codex-config-path
type: summary
status: unverified
title: codex::config_path
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::config_path
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::config_path@774bfa291cfcc73c
map_version: '2'
---
# codex::config_path

Returns the path of the Codex TOML config, <root>/.codex/config.toml, which holds the MCP block. It does no I/O.

## Behaviour

- /repo -> /repo/.codex/config.toml
- file need not exist

## Terms

toml config, mcp config file, codex settings file

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:39-41`
- **Signature:** `fn config_path(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[codex-codex-dir]]
- **Called by:** [[codex-write-mcp]], [[codex-install]], [[codex-uninstall]], [[codex-inspect]]
- **Tests:** `install_preserves_config_and_uninstalls_cleanly`
