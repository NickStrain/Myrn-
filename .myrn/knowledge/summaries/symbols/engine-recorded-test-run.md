---
id: engine-recorded-test-run
type: summary
status: unverified
title: Engine::recorded_test_run
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::recorded_test_run
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::recorded_test_run@1bec1f3fbfa5fecd
---
# Engine::recorded_test_run

Takes a command, reduces it to its test step, and scans the 20 most recent episode files for a test-flagged entry whose command shares that step in either direction. It returns the recorded step text, or None if no session ran a matching test.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:184-204`
- **Signature:** `fn recorded_test_run(&self, cmd: &str) -> Result<Option<String>>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[checks-test-step]], [[episodes-read]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[engine-check-evidence]]
