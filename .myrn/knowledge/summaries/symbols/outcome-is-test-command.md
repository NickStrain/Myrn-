---
id: outcome-is-test-command
type: summary
status: unverified
title: outcome::is_test_command
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/outcome.rs#capture::outcome::is_test_command
anchors:
- sym:crates/myrn-engine/src/capture/outcome.rs#capture::outcome::is_test_command@19939fdbe6faeeef
map_version: '2'
---
# outcome::is_test_command

Returns true if any segment of a shell command, split on & ; | newline and (, starts with one of the RUNNERS word sequences such as cargo test, pytest or npm test.

## Behaviour

- 'cd crates/x && cargo test' -> true (the segment after && is checked)
- 'RUST_BACKTRACE=1 cargo nextest run' -> true (leading KEY=value words are skipped); '--x=1 cargo test' -> false (words starting with '-' are not skipped)
- 'cargo testify', 'cat pytest.ini', 'echo npm', 'git commit -m \'cargo test passes\'' -> false (whole-word match on the leading words only; quotes are not parsed)

## Terms

test run, test runner, runner invocation, test detection, command chaining, env assignment prefix, pass fail source, test command

## Connections

- **Location:** `crates/myrn-engine/src/capture/outcome.rs:47-58`
- **Signature:** `pub fn is_test_command(command: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-outcome-rs]]
- **Called by:** [[capture-to-entry]], [[checks-test-step]]
- **Tests:** `recognises_test_runs`
