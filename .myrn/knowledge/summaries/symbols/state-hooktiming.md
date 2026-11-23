---
id: state-hooktiming
type: summary
status: unverified
title: state::HookTiming
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::HookTiming
anchors:
- sym:crates/myrn-store/src/state.rs#state::HookTiming@bbea72c3a5a7444f
map_version: '2'
---
# state::HookTiming

Stores one hook run: which agent and event fired, how many milliseconds it took, and a timestamp.

## Behaviour

- millis is u64, so a negative duration cannot be represented
- time is i64 and its unit is not stated in this file (the caller sets it; tests use 0)
- no serde default on any field, so a stored timing missing a field fails to load the whole State

## Terms

hook timing, hook latency, hook duration, run time, milliseconds, timing record, hook event, agent event log

## Connections

- **Location:** `crates/myrn-store/src/state.rs:42-47`
- **Signature:** `pub struct HookTiming`
- **In:** [[file-crates-myrn-store-src-state-rs]]
