---
id: condition-as-str
type: summary
status: unverified
title: Condition::as_str
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Condition::as_str
anchors:
- sym:bench/src/runner.rs#runner::Condition::as_str@6c35d38214455592
map_version: '2'
---
# Condition::as_str

Returns the lowercase name of the condition, used in file names, the store directory name and CLI arguments.

## Behaviour

- Without -> "without", Off -> "off", With -> "with", Fixed -> "fixed"
- Inverse of parse for every variant
- Result is a static string, so nothing is allocated

## Terms

lowercase name, file name part, slug, short name, condition id

## Connections

- **Location:** `bench/src/runner.rs:37-44`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[rescore-runs]], [[report-compare-by]], [[report-render]], [[runner-run-inner]], [[runner-export-records]]
