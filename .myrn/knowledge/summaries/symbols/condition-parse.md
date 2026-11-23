---
id: condition-parse
type: summary
status: unverified
title: Condition::parse
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Condition::parse
anchors:
- sym:bench/src/runner.rs#runner::Condition::parse@c61dd4749686f4de
map_version: '2'
---
# Condition::parse

Turns a condition name into a Condition, and fails with a message listing the four accepted names otherwise.

## Behaviour

- "fixed" -> Ok(Fixed)
- "With" or "" -> error, because matching is exact and case-sensitive
- Unknown string -> error that names the accepted values

## Terms

from string, command line argument, case sensitive, validation, parse error, unknown name

## Connections

- **Location:** `bench/src/runner.rs:57-65`
- **Signature:** `pub fn parse(s: &str) -> Result<Condition>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[real-main]]
