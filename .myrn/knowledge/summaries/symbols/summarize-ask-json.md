---
id: summarize-ask-json
type: summary
status: unverified
title: summarize::ask_json
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::ask_json
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::ask_json@8911b0bca1a75149
---
# summarize::ask_json

Runs the model command with the prompt and parses the first JSON object in its answer, retrying the identical prompt once only when the answer is not parseable, with the cost of both calls summed. A failure of the command itself is returned at once without retry, and a second parse failure is returned as an error.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:750-760`
- **Signature:** `fn ask_json(command: &[String], prompt: &str) -> Result<(serde_json::Value, f64)>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[summarize-run-model]], [[summarize-parse-json]]
- **Called by:** [[summarize-summarize-file]], [[summarize-summarize-dir]]
