---
id: env-hook
type: summary
status: unverified
title: Env::hook
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#Env::hook
anchors:
- sym:crates/myrn/tests/connect.rs#Env::hook@68d21be8cede8ff7
map_version: '2'
---
# Env::hook

Runs myrn hook with the agent and event, from the given directory, writes the payload to stdin, and returns the Output plus elapsed milliseconds.

## Behaviour

- agent 'claude', event 'SessionStart', dir X -> runs myrn hook claude SessionStart with cwd X
- stdin is closed after the payload is written, so the child reads EOF
- elapsed time starts before spawn, so it includes process startup

## Terms

agent hook, stdin payload, hook latency, elapsed time, hook event, JSON event

## Connections

- **Location:** `crates/myrn/tests/connect.rs:61-79`
- **Signature:** `fn hook(&self, agent: &str, event: &str, payload: &str, dir: &Path) -> (Output, u128)`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Calls:** [[env-cmd]]
- **Tests:** `connect_hooks_doctor_disconnect`, `sessions_are_recorded_redacted_and_listed`
