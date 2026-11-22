---
id: model
type: summary
status: unverified
title: model
source: myrn summarize
about: sym:crates/myrn-engine/tests/summarize.rs#model
anchors:
- sym:crates/myrn-engine/tests/summarize.rs#model@ca120e14b8a3c191
---
# model

Returns the argv for a stub summarizer: sh -c that discards stdin, appends a line to calls.log in tmp, and prints a fixed JSON answer. The answer gives the same file summary, the symbol summaries 'grace' and 'Vault', and a project summary, so it serves both files and directories.

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs:38-49`
- **Signature:** `fn model(tmp: &Path) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-tests-summarize-rs]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
