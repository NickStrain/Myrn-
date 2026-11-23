---
id: condition-label
type: summary
status: unverified
title: Condition::label
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Condition::label
anchors:
- sym:bench/src/runner.rs#runner::Condition::label@1212495ef5a29aa7
map_version: '2'
---
# Condition::label

Returns the human-readable column heading for the condition in the report.

## Behaviour

- Without -> "Without myrn"
- Fixed -> "myrn, inject on, fixed ranking"
- Off and With are "myrn, inject off" and "myrn, inject on"

## Terms

column header, table heading, display name, report label, caption

## Connections

- **Location:** `bench/src/runner.rs:46-53`
- **Signature:** `pub fn label(self) -> &'static str`
- **In:** [[file-bench-src-runner-rs]]
