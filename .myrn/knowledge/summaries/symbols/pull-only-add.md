---
id: pull-only-add
type: summary
status: unverified
title: pull_only::add
source: myrn summarize
about: sym:crates/myrn-adapters/src/pull_only.rs#pull_only::add
anchors:
- sym:crates/myrn-adapters/src/pull_only.rs#pull_only::add@881790d811e1d6fb
map_version: '2'
---
# pull_only::add

Inserts the marker block at the end of the file, replacing any existing block so it is idempotent. It creates parent directories and the file if needed, and returns true only when the file did not exist before.

## Behaviour

- missing file -> created with just the block, returns true
- existing block -> removed and re-appended at the end, returns false
- body ending in a newline gets one extra newline before the block; body ending in a blank line gets none

## Terms

insert block, install, append block, refresh block, create file, upsert, idempotent

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs:23-39`
- **Signature:** `pub fn add(path: &Path) -> Result<bool>`
- **In:** [[file-crates-myrn-adapters-src-pull-only-rs]]
- **Calls:** [[pull-only-block]], [[pull-only-strip]]
- **Called by:** [[claudecode-install]], [[codex-install]]
- **Tests:** `add_and_remove_are_clean`
