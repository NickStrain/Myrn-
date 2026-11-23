---
id: write-history
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/history.rs#write
anchors:
- sym:crates/myrn-engine/tests/history.rs#write@bf0595a78a1deb5b
map_version: '2'
---
# write

Writes each (relative path, content) pair under root, creating missing parent directories and overwriting existing files. Panics on any I/O error.

## Behaviour

- [("a/b.txt", "x")] -> creates root/a/b.txt containing x, creating a/ if needed
- existing file at the path -> its content is replaced
- empty slice -> no-op

## Terms

create files, write fixtures, mkdir -p, nested paths, test helper, overwrite file

## Connections

- **Location:** `crates/myrn-engine/tests/history.rs:11-17`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-history-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[setup-history]]
- **Tests:** `log_lists_records_and_trace_shows_where_they_came_from`, `diff_compares_shared_knowledge_between_points`
