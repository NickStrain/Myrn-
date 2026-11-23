---
id: engine-passing-test-runs
type: summary
status: unverified
title: Engine::passing_test_runs
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::passing_test_runs
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::passing_test_runs@1adef5415dc2c022
map_version: '2'
---
# Engine::passing_test_runs

Collects the command strings of recorded episode steps whose test result is pass.

## Behaviour

- Episode step with test pass and a command -> command included
- Step with no command, or a non-pass result -> skipped

## Terms

passing tests, test runs, episode log, green tests

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:243-255`
- **Signature:** `fn passing_test_runs(&self) -> Result<Vec<String>>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Calls:** [[episodes-read]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[engine-promote]]
