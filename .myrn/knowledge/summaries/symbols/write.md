---
id: write
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#write
anchors:
- sym:crates/myrn-engine/tests/notes.rs#write@bf0595a78a1deb5b
map_version: '2'
---
# write

Writes each (relative path, content) pair under root, creating missing parent directories and overwriting existing files.

## Behaviour

- root plus ('a/b.md', 'x') -> creates a/ under root and writes b.md containing 'x'
- existing file at the path -> replaced with the new content
- any I/O failure -> panics through unwrap

## Terms

create files, mkdir parents, seed files, write fixtures, overwrite file

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:30-36`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-notes]]
- **Tests:** `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `links_and_summaries_survive_code_changes`
