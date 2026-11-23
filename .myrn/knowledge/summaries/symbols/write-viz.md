---
id: write-viz
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/viz.rs#write
anchors:
- sym:crates/myrn-engine/tests/viz.rs#write@bf0595a78a1deb5b
map_version: '2'
---
# write

Writes each (relative path, content) pair under root, creating parent directories as needed. Panics on any I/O error because it uses unwrap.

## Behaviour

- write(root, [('a/b.txt', 'x')]) -> creates root/a/ and writes root/a/b.txt containing 'x'
- write(root, []) -> no-op
- existing file at a path -> overwritten with the new content

## Terms

fixture files, test setup, mkdir -p, create directories, write files, test data

## Connections

- **Location:** `crates/myrn-engine/tests/viz.rs:8-14`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-viz-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-viz]]
