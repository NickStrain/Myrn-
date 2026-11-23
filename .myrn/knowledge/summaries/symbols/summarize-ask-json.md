---
id: summarize-ask-json
type: summary
status: unverified
title: summarize::ask_json
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::ask_json
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::ask_json@8911b0bca1a75149
map_version: '2'
---
# summarize::ask_json

Runs run_model and parses the answer as JSON, asking once more if the first answer does not parse, and returns the value with the summed cost.

## Behaviour

- first answer parses -> one call, its cost returned
- first answer fails to parse -> second call; if that also fails, its parse error is returned
- an error from run_model on either call is returned at once; a retry happens only on a parse failure

## Terms

json retry, model answer, cost sum, answer parsing, retry once

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:837-847`
- **Signature:** `fn ask_json(command: &[String], prompt: &str) -> Result<(serde_json::Value, f64)>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[summarize-run-model]], [[summarize-parse-json]]
- **Called by:** [[summarize-summarize-file]], [[summarize-summarize-dir]]
