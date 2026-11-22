---
id: codex-hooks-path
type: summary
status: unverified
title: codex::hooks_path
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::hooks_path
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::hooks_path@bb051ae9ae5e184d
---
# codex::hooks_path

Returns root/.codex/hooks.json, where myrn writes its hook entries.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:36-38`
- **Signature:** `fn hooks_path(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[codex-codex-dir]]
- **Called by:** [[codex-install]], [[codex-uninstall]], [[codex-inspect]]
- **Tests:** `install_preserves_config_and_uninstalls_cleanly`, `pull_only_uses_agents_md`
