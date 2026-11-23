---
id: checks-test-step
type: summary
status: unverified
title: checks::test_step
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::test_step
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::test_step@8a49ccd8afd41d0a
map_version: '2'
---
# checks::test_step

Extracts the bare test command from a shell line, dropping cd, pipes, redirections and leading env assignments.

## Behaviour

- "cd /x && cargo test -p a 2>&1 | grep ok" -> "cargo test -p a"
- "RUST_LOG=1 pytest tests/a.py -q" -> "pytest tests/a.py -q"
- "ls && cat x" -> None, because no segment is a test command

## Terms

test command, test step, shell line, bare command, env prefix, redirection, pipe

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:52-63`
- **Signature:** `pub fn test_step(cmd: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[outcome-is-test-command]]
- **Called by:** [[engine-recorded-test-run]], [[engine-records-request]], [[sharing-clean]], [[engine-verified-by-test]]
- **Tests:** `test_steps`
