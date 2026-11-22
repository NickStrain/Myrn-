---
id: state-state
type: summary
status: unverified
title: state::State
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::State
anchors:
- sym:crates/myrn-store/src/state.rs#state::State@cba83df8f4cfb8ee
---
# state::State

Persisted per-project state: capture_consent flag, consent_time as Unix seconds, agents keyed by adapter name, and hook_timings capped at 50. Every field defaults when absent from the JSON, so older or partial files still load.

## Connections

- **Location:** `crates/myrn-store/src/state.rs:11-24`
- **Signature:** `pub struct State`
- **In:** [[file-crates-myrn-store-src-state-rs]]
