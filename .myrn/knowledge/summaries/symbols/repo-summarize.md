---
id: repo-summarize
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/summarize.rs#repo
anchors:
- sym:crates/myrn-engine/tests/summarize.rs#repo@ca818524d3ba37df
---
# repo

Creates a temp dir with a repo/ subdirectory holding the given files, git-inits it, and commits everything as one commit 'first'. Returns the TempDir (must stay alive) and an Engine initialized on that root with its home under the temp dir. Panics on any failure.

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs:19-34`
- **Signature:** `fn repo(files: &[(&str, &str)]) -> (tempfile::TempDir, Engine)`
- **In:** [[file-crates-myrn-engine-tests-summarize-rs]]
- **Calls:** [[engine-init]], [[write-summarize]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
