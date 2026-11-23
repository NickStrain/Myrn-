---
id: pull-only-remove
type: summary
status: unverified
title: pull_only::remove
source: myrn summarize
about: sym:crates/myrn-adapters/src/pull_only.rs#pull_only::remove
anchors:
- sym:crates/myrn-adapters/src/pull_only.rs#pull_only::remove@e899c337db615901
map_version: '2'
---
# pull_only::remove

Removes the marker block from the file and writes the rest back. It can delete the file when myrn created it and nothing else is left. It returns true when it writes or deletes, and false when there is nothing to do.

## Behaviour

- missing file or no START marker -> false, file untouched
- delete_if_empty true and remaining text is blank -> file deleted, returns true
- START present but END missing -> text written back unchanged, still returns true

## Terms

uninstall, delete block, clean up, delete file if empty, remove marker, revert

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs:43-57`
- **Signature:** `pub fn remove(path: &Path, delete_if_empty: bool) -> Result<bool>`
- **In:** [[file-crates-myrn-adapters-src-pull-only-rs]]
- **Calls:** [[pull-only-strip]]
- **Called by:** [[claudecode-uninstall]], [[codex-uninstall]]
- **Tests:** `add_and_remove_are_clean`
