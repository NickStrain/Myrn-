---
id: setup-connect
type: summary
status: unverified
title: setup
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#setup
anchors:
- sym:crates/myrn/tests/connect.rs#setup@f1fcb3dd8220a59a
map_version: '2'
---
# setup

Creates a throwaway git repo with a seeded src/lib.rs, makes its first commit, runs myrn init in it, and returns the Env.

## Behaviour

- src/lib.rs has refresh_session calling validate_token, with a doc comment on refresh_session
- first commit has no parent, since the parents slice is empty
- panics if myrn init exits non-zero

## Terms

fixture, seed repo, git init, initial commit, myrn init, test project

## Connections

- **Location:** `crates/myrn/tests/connect.rs:17-43`
- **Signature:** `fn setup() -> Env`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Calls:** [[env-myrn]]
- **Tests:** `connect_hooks_doctor_disconnect`, `sessions_are_recorded_redacted_and_listed`
