---
id: tests-payload
type: summary
status: unverified
title: tests::payload
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::tests::payload
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::tests::payload@89fa4047e7ff9e14
map_version: '2'
---
# tests::payload

Test helper that loads a recorded hook payload by file name from tests/fixtures/hook-payloads/claude-code and parses it as JSON.

## Behaviour

- session_start.json -> parsed Value from that fixture file
- Missing file or invalid JSON -> panic through unwrap

## Terms

fixture loader, recorded payload, sample hook input, test data

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:396-401`
- **Signature:** `fn payload(name: &str) -> Value`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Tests:** `translates_recorded_payloads`
