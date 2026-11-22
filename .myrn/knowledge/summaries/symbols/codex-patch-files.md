---
id: codex-patch-files
type: summary
status: unverified
title: codex::patch_files
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::patch_files
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::patch_files@e1f93c6c02057d2e
---
# codex::patch_files

Extracts unique file paths from an apply_patch body by reading the Add File, Update File, Delete File, and Move to header lines, in order of first appearance.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:108-127`
- **Signature:** `pub fn patch_files(patch: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-translate]]
- **Tests:** `patch_file_names`
