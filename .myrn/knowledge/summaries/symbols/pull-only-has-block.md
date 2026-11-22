---
id: pull-only-has-block
type: summary
status: unverified
title: pull_only::has_block
source: myrn summarize
about: sym:crates/myrn-adapters/src/pull_only.rs#pull_only::has_block
anchors:
- sym:crates/myrn-adapters/src/pull_only.rs#pull_only::has_block@3d01d86b5e49e8b0
---
# pull_only::has_block

Returns true only if the file at path can be read and contains both START and END markers; a missing or unreadable file returns false. It does not check that END comes after START.

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs:18-20`
- **Signature:** `pub fn has_block(path: &Path) -> bool`
- **In:** [[file-crates-myrn-adapters-src-pull-only-rs]]
- **Called by:** [[claudecode-inspect]], [[codex-inspect]]
- **Tests:** `pull_only_writes_instruction_block_not_hooks`, `pull_only_uses_agents_md`
