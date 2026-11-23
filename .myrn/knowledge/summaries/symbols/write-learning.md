---
id: write-learning
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/learning.rs#write
anchors:
- sym:crates/myrn-engine/tests/learning.rs#write@bf0595a78a1deb5b
map_version: '2'
---
# write

Writes each (relative path, content) pair under the given root, creating missing parent directories first. Panics on any I/O error.

## Behaviour

- root R and [("a/b.py", "x")] -> creates R/a/b.py containing x
- existing file at the path -> overwritten with the new content
- empty files slice -> nothing is created

## Terms

create files, fixture, make directories, overwrite file, test setup, write source

## Connections

- **Location:** `crates/myrn-engine/tests/learning.rs:11-17`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-learning-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-learning]]
