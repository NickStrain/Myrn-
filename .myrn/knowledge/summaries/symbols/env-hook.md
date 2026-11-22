---
id: env-hook
type: summary
status: unverified
title: Env::hook
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#Env::hook
anchors:
- sym:crates/myrn/tests/connect.rs#Env::hook@68d21be8cede8ff7
---
# Env::hook

Spawns myrn hook <agent> <event> from the given directory, writes the payload to stdin, and returns the captured Output plus elapsed milliseconds.

## Connections

- **Location:** `crates/myrn/tests/connect.rs:61-79`
- **Signature:** `fn hook(&self, agent: &str, event: &str, payload: &str, dir: &Path) -> (Output, u128)`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Calls:** [[env-cmd]]
- **Tests:** `connect_hooks_doctor_disconnect`, `sessions_are_recorded_redacted_and_listed`
