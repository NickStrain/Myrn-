---
id: copy-dir
type: summary
status: unverified
title: copy_dir
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#copy_dir
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#copy_dir@cf8159461c045252
map_version: '2'
---
# copy_dir

Recursively copies a directory tree from one path to another, creating directories as needed. Panics on any IO error.

## Behaviour

- from has a/b.txt -> to/a/b.txt is created with the same contents
- empty from -> only the to directory is created
- existing files in to are overwritten by std::fs::copy

## Terms

recursive copy, duplicate tree, mirror directory, clone files

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:17-28`
- **Signature:** `fn copy_dir(from: &Path, to: &Path)`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[copy-dir]]
- **Called by:** [[copy-dir]], [[setup]]
