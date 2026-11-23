---
id: env-myrn
type: summary
status: unverified
title: Env::myrn
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#Env::myrn
anchors:
- sym:crates/myrn/tests/connect.rs#Env::myrn@0206ad82ee800447
map_version: '2'
---
# Env::myrn

Runs myrn with the given args to completion and returns the captured stdout, stderr, and exit status.

## Behaviour

- args ['doctor'] -> Output with the doctor report on stdout
- a non-zero exit is returned, not treated as an error; callers assert it
- panics only if the binary cannot be spawned

## Terms

run binary, capture output, blocking run, exit status, subprocess

## Connections

- **Location:** `crates/myrn/tests/connect.rs:56-58`
- **Signature:** `fn myrn(&self, args: &[&str]) -> Output`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Calls:** [[env-cmd]]
- **Called by:** [[setup-connect]]
- **Tests:** `connect_hooks_doctor_disconnect`, `sessions_are_recorded_redacted_and_listed`
