---
id: env-myrn
type: summary
status: unverified
title: Env::myrn
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#Env::myrn
anchors:
- sym:crates/myrn/tests/connect.rs#Env::myrn@0206ad82ee800447
---
# Env::myrn

Runs myrn with the given args through cmd and returns its captured Output, panicking if the process cannot be spawned.

## Connections

- **Location:** `crates/myrn/tests/connect.rs:56-58`
- **Signature:** `fn myrn(&self, args: &[&str]) -> Output`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Calls:** [[env-cmd]]
- **Called by:** [[setup-connect]]
- **Tests:** `connect_hooks_doctor_disconnect`, `sessions_are_recorded_redacted_and_listed`
