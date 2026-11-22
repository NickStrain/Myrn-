---
id: setup-connect
type: summary
status: unverified
title: setup
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#setup
anchors:
- sym:crates/myrn/tests/connect.rs#setup@f1fcb3dd8220a59a
---
# setup

Creates a temp git repo with src/lib.rs (refresh_session calls validate_token), makes one commit, runs myrn init, and asserts it succeeds. Returns the Env and panics on any failure.

## Connections

- **Location:** `crates/myrn/tests/connect.rs:17-43`
- **Signature:** `fn setup() -> Env`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Calls:** [[env-myrn]]
- **Tests:** `connect_hooks_doctor_disconnect`, `sessions_are_recorded_redacted_and_listed`
