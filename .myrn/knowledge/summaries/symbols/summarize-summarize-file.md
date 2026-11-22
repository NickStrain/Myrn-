---
id: summarize-summarize-file
type: summary
status: unverified
title: summarize::summarize_file
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::summarize_file
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::summarize_file@09649d216acc6b3c
---
# summarize::summarize_file

Builds one prompt for a source file (cut to 160000 bytes at a char boundary) that lists the stale symbols as the required keys of the symbols object, with qualified name, kind and line range, and returns the cleaned file summary, a map from symbol qname to summary (non-string values are dropped) and the model cost. Read and model errors propagate, and the caller matches the returned keys against qualified names exactly.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:616-663`
- **Signature:** `fn summarize_file(command: &[String], cwd: &Path, job: &FileJob) -> Result<FileOut>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[summarize-ask-json]], [[summarize-clean]]
- **Called by:** [[engine-summarize]]
