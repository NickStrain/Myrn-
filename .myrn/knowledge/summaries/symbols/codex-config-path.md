---
id: codex-config-path
type: summary
status: unverified
title: codex::config_path
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::config_path
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::config_path@774bfa291cfcc73c
---
# codex::config_path

Returns root/.codex/config.toml, which receives the managed [mcp_servers.myrn] block.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:39-41`
- **Signature:** `fn config_path(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[codex-codex-dir]]
- **Called by:** [[codex-write-mcp]], [[codex-install]], [[codex-uninstall]], [[codex-inspect]]
- **Tests:** `install_preserves_config_and_uninstalls_cleanly`
