---
id: tests-ctx-codex
type: summary
status: unverified
title: tests::ctx
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::tests::ctx
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::tests::ctx@234c17d2fdb6a41c
---
# tests::ctx

Test helper that builds an InstallCtx for a root with the binary path '/opt/my rn/myrn', which exercises quoting because the path contains a space.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:415-421`
- **Signature:** `fn ctx(root: &Path) -> InstallCtx`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Tests:** `install_preserves_config_and_uninstalls_cleanly`, `pull_only_uses_agents_md`
