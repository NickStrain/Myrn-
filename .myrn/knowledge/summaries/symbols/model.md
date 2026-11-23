---
id: model
type: summary
status: unverified
title: model
source: myrn summarize
about: sym:crates/myrn-engine/tests/summarize.rs#model
anchors:
- sym:crates/myrn-engine/tests/summarize.rs#model@ca120e14b8a3c191
map_version: '2'
---
# model

Returns a shell argv (sh -c script) that stands in for the model: it drains stdin, appends a line to tmp/calls.log, and prints one fixed JSON answer.

## Behaviour

- any call -> same answer: file summary, symbols grace and Vault, summary "Credential handling."
- each invocation -> one new line in calls.log
- returns Vec<String> argv, not a model object

## Terms

fake model, stub model, mock llm, stand-in model, canned answer, model command

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs:38-49`
- **Signature:** `fn model(tmp: &Path) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-tests-summarize-rs]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
