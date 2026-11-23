---
id: codex-patch-files
type: summary
status: unverified
title: codex::patch_files
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::patch_files
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::patch_files@e1f93c6c02057d2e
map_version: '2'
---
# codex::patch_files

Lists the unique file paths named by Add File, Update File, Delete File and Move to lines in an apply_patch body, in first-seen order.

## Behaviour

- '*** Update File: a.rs' line -> ['a.rs']
- the same path named twice -> listed once
- no marker lines -> empty list; paths are trimmed

## Terms

apply_patch files, patch targets, edited paths, deduplicated paths

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:108-127`
- **Signature:** `pub fn patch_files(patch: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-translate]]
- **Tests:** `patch_file_names`
