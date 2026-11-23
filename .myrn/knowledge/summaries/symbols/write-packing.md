---
id: write-packing
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/packing.rs#write
anchors:
- sym:crates/myrn-engine/tests/packing.rs#write@bf0595a78a1deb5b
map_version: '2'
---
# write

Writes each (relative path, contents) pair under root, creating parent directories first. Panics on any I/O error.

## Behaviour

- [('a/b.py', 'x')] -> creates a/ and writes a/b.py containing x
- empty slice -> no-op
- existing file at the same path -> overwritten

## Terms

fixture, scaffold, create files, mkdir, seed files, test setup, write to disk

## Connections

- **Location:** `crates/myrn-engine/tests/packing.rs:9-15`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-packing-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-packing]]
