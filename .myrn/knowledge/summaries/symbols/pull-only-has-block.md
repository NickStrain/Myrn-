---
id: pull-only-has-block
type: summary
status: unverified
title: pull_only::has_block
source: myrn summarize
about: sym:crates/myrn-adapters/src/pull_only.rs#pull_only::has_block
anchors:
- sym:crates/myrn-adapters/src/pull_only.rs#pull_only::has_block@3d01d86b5e49e8b0
map_version: '2'
---
# pull_only::has_block

Returns true if the file can be read and contains both the START and END marker strings anywhere in it. It does not check the text between them or whether they are in order.

## Behaviour

- missing or unreadable file -> false
- START present but END missing -> false
- both markers present in any order -> true

## Terms

marker check, block present, detect block, already installed, contains markers

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs:18-20`
- **Signature:** `pub fn has_block(path: &Path) -> bool`
- **In:** [[file-crates-myrn-adapters-src-pull-only-rs]]
- **Called by:** [[claudecode-inspect]], [[codex-inspect]]
- **Tests:** `pull_only_writes_instruction_block_not_hooks`, `pull_only_uses_agents_md`
