---
id: write-summarize
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/summarize.rs#write
anchors:
- sym:crates/myrn-engine/tests/summarize.rs#write@bf0595a78a1deb5b
map_version: '2'
---
# write

Creates each listed file under root, making parent directories as needed, and overwrites existing files.

## Behaviour

- [("a/b.rs", "x")] -> creates root/a/ and writes root/a/b.rs with content x
- existing file -> content replaced, no error
- IO failure -> panics via unwrap

## Terms

fixture files, seed files, create directories, overwrite file, test setup

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs:11-17`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-summarize-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-summarize]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
