---
id: pull-only-add
type: summary
status: unverified
title: pull_only::add
source: myrn summarize
about: sym:crates/myrn-adapters/src/pull_only.rs#pull_only::add
anchors:
- sym:crates/myrn-adapters/src/pull_only.rs#pull_only::add@881790d811e1d6fb
---
# pull_only::add

Strips any existing block from the file (or empty content if the file is missing), appends a fresh block with a blank-line separator, creates parent directories, and overwrites the file. Returns true only when the file did not exist before; a re-run refreshes the block in place at the end of the file.

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs:23-39`
- **Signature:** `pub fn add(path: &Path) -> Result<bool>`
- **In:** [[file-crates-myrn-adapters-src-pull-only-rs]]
- **Calls:** [[pull-only-block]], [[pull-only-strip]]
- **Called by:** [[claudecode-install]], [[codex-install]]
- **Tests:** `add_and_remove_are_clean`
