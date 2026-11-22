---
id: condition-parse
type: summary
status: unverified
title: Condition::parse
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Condition::parse
anchors:
- sym:bench/src/runner.rs#runner::Condition::parse@8ac52496bcb3ffd1
---
# Condition::parse

Converts 'without', 'off' or 'with' into a Condition. Any other string returns an error that lists the valid names.

## Connections

- **Location:** `bench/src/runner.rs:53-60`
- **Signature:** `pub fn parse(s: &str) -> Result<Condition>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[real-main]]
