---
id: env-settings
type: summary
status: unverified
title: Env::settings
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#Env::settings
anchors:
- sym:crates/myrn/tests/connect.rs#Env::settings@3a3ecc88ac61ceb7
map_version: '2'
---
# Env::settings

Reads .claude/settings.local.json in the repo and parses it into a serde_json Value.

## Behaviour

- returns the whole file, not only the hooks key
- panics if the file is missing
- panics if the file is not valid JSON

## Terms

settings.local.json, claude settings, hooks config, parse config, read config

## Connections

- **Location:** `crates/myrn/tests/connect.rs:81-86`
- **Signature:** `fn settings(&self) -> Value`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Tests:** `connect_hooks_doctor_disconnect`
