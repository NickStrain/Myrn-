---
id: codex-hooks-path
type: summary
status: unverified
title: codex::hooks_path
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::hooks_path
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::hooks_path@bb051ae9ae5e184d
map_version: '2'
---
# codex::hooks_path

Returns the path of the Codex hooks file, <root>/.codex/hooks.json. It does no I/O.

## Behaviour

- /repo -> /repo/.codex/hooks.json
- file need not exist

## Terms

hooks file, hook config, hooks json location

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:36-38`
- **Signature:** `fn hooks_path(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[codex-codex-dir]]
- **Called by:** [[codex-install]], [[codex-uninstall]], [[codex-inspect]]
- **Tests:** `install_preserves_config_and_uninstalls_cleanly`, `pull_only_uses_agents_md`
