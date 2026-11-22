---
id: checks-test-step
type: summary
status: unverified
title: checks::test_step
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::test_step
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::test_step@8a49ccd8afd41d0a
---
# checks::test_step

Splits a shell line on &, ;, |, and newlines, finds the first segment that is a test command, and returns it with leading env assignments and redirections removed, or None if no segment is a test command.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:52-63`
- **Signature:** `pub fn test_step(cmd: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[outcome-is-test-command]]
- **Called by:** [[engine-recorded-test-run]], [[engine-records-request]]
- **Tests:** `test_steps`
