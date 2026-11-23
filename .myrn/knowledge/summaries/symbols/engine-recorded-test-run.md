---
id: engine-recorded-test-run
type: summary
status: unverified
title: Engine::recorded_test_run
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::recorded_test_run
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::recorded_test_run@1bec1f3fbfa5fecd
map_version: '2'
---
# Engine::recorded_test_run

Finds a test command in the 20 most recent sessions that matches the cited command and returns the recorded test step.

## Behaviour

- cited command with no test step -> None without reading sessions
- a recorded test step that contains the cited step, or is contained in it -> that recorded step is returned
- no recorded session has a matching test command -> None

## Terms

recorded session, episode, test command match, session log, test_run evidence

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:184-204`
- **Signature:** `fn recorded_test_run(&self, cmd: &str) -> Result<Option<String>>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[checks-test-step]], [[episodes-read]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[engine-check-evidence]]
