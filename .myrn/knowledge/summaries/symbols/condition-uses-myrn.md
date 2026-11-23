---
id: condition-uses-myrn
type: summary
status: unverified
title: Condition::uses_myrn
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Condition::uses_myrn
anchors:
- sym:bench/src/runner.rs#runner::Condition::uses_myrn@8e04592adc8a61f5
map_version: '2'
---
# Condition::uses_myrn

Returns true when myrn is connected for this condition, which is every condition except Without.

## Behaviour

- Without -> false
- Off, With and Fixed -> true
- Controls whether injections and records are collected after the run

## Terms

myrn enabled, connected, has myrn, mcp enabled, myrn flag

## Connections

- **Location:** `bench/src/runner.rs:54-56`
- **Signature:** `pub fn uses_myrn(self) -> bool`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[real-main]], [[report-render]], [[runner-run-inner]]
