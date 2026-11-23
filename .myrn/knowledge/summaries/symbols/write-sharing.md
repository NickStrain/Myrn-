---
id: write-sharing
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/sharing.rs#write
anchors:
- sym:crates/myrn-engine/tests/sharing.rs#write@bf0595a78a1deb5b
map_version: '2'
---
# write

Writes each (relative path, content) pair under root, creating missing parent directories and overwriting existing files. Panics on any I/O error.

## Behaviour

- files with pkg/x.py creates the pkg directory first
- existing file at the same path is replaced, not appended
- any failed create or write panics via unwrap

## Terms

create files, save to disk, write files, make directories, overwrite

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs:22-28`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-sharing-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[two-clones]]
