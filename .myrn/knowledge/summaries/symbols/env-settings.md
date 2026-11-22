---
id: env-settings
type: summary
status: unverified
title: Env::settings
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#Env::settings
anchors:
- sym:crates/myrn/tests/connect.rs#Env::settings@3a3ecc88ac61ceb7
---
# Env::settings

Reads and parses .claude/settings.local.json in the repo as JSON, panicking if the file is missing or not valid JSON.

## Connections

- **Location:** `crates/myrn/tests/connect.rs:81-86`
- **Signature:** `fn settings(&self) -> Value`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Tests:** `connect_hooks_doctor_disconnect`
