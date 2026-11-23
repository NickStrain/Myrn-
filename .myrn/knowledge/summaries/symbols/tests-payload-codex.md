---
id: tests-payload-codex
type: summary
status: unverified
title: tests::payload
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::tests::payload
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::tests::payload@767ae90572a7aef7
map_version: '2'
---
# tests::payload

Test helper that reads and parses a JSON fixture from tests/fixtures/hook-payloads/codex/ relative to CARGO_MANIFEST_DIR.

## Behaviour

- 'stop.json' -> the parsed fixture file
- missing file or invalid JSON -> panics through unwrap

## Terms

fixture loader, recorded hook input, sample event data

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:473-478`
- **Signature:** `fn payload(name: &str) -> Value`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Tests:** `translates_recorded_payloads`
