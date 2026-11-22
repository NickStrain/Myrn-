---
id: tests-ctx
type: summary
status: unverified
title: tests::ctx
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::tests::ctx
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::tests::ctx@a1a83742755497b9
---
# tests::ctx

Test helper that builds an InstallCtx at the given root with binary /opt/myrn/bin/myrn and run_agent_cli false, so tests never invoke the real claude CLI.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:328-334`
- **Signature:** `fn ctx(root: &Path) -> InstallCtx`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Tests:** `install_is_idempotent_and_keeps_other_hooks`, `pull_only_writes_instruction_block_not_hooks`
