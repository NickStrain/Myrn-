---
id: write-summarize
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/summarize.rs#write
anchors:
- sym:crates/myrn-engine/tests/summarize.rs#write@bf0595a78a1deb5b
---
# write

Writes each (relative path, content) pair under root, creating parent directories and overwriting existing files. Panics on any I/O error. Used to seed the fixture repo and to edit files mid-test.

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs:11-17`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-summarize-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-summarize]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
