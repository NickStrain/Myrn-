---
id: summarize-summarize-dir
type: summary
status: unverified
title: summarize::summarize_dir
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::summarize_dir
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::summarize_dir@4d6a4cad30f1ee1c
---
# summarize::summarize_dir

Asks the model for a one-to-three-sentence summary of a directory from its child listing (the project root also gets a README excerpt) and returns `(summary, cost)`. It errors if the answer has no non-empty `summary` field.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:665-699`
- **Signature:** `fn summarize_dir( command: &[String], _cwd: &Path, job: &DirJob, readme: Option<&str>, ) -> Result<(String, f64)>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[summarize-ask-json]], [[summarize-clean]]
- **Called by:** [[engine-summarize]]
