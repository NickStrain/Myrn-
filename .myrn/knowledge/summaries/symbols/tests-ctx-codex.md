---
id: tests-ctx-codex
type: summary
status: unverified
title: tests::ctx
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::tests::ctx
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::tests::ctx@234c17d2fdb6a41c
map_version: '2'
---
# tests::ctx

Test helper that builds an InstallCtx with the given repo root, the binary path '/opt/my rn/myrn' (contains a space) and run_agent_cli set to false.

## Behaviour

- root given -> repo_root equals that root
- binary is always '/opt/my rn/myrn'
- run_agent_cli is always false

## Terms

test fixture, install context, spaced path, test setup

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:415-421`
- **Signature:** `fn ctx(root: &Path) -> InstallCtx`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Tests:** `install_preserves_config_and_uninstalls_cleanly`, `pull_only_uses_agents_md`
