---
id: pull-only-remove
type: summary
status: unverified
title: pull_only::remove
source: myrn summarize
about: sym:crates/myrn-adapters/src/pull_only.rs#pull_only::remove
anchors:
- sym:crates/myrn-adapters/src/pull_only.rs#pull_only::remove@e899c337db615901
---
# pull_only::remove

Returns false if the file is missing or has no START marker. Otherwise it strips the block and either deletes the file (when delete_if_empty is set and the remaining text is blank) or rewrites it, returning true. If START exists but END does not, strip returns the text unchanged, so the file is rewritten identically and true is still returned.

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs:43-57`
- **Signature:** `pub fn remove(path: &Path, delete_if_empty: bool) -> Result<bool>`
- **In:** [[file-crates-myrn-adapters-src-pull-only-rs]]
- **Calls:** [[pull-only-strip]]
- **Called by:** [[claudecode-uninstall]], [[codex-uninstall]]
- **Tests:** `add_and_remove_are_clean`
