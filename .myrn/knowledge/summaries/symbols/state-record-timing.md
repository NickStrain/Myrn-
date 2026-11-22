---
id: state-record-timing
type: summary
status: unverified
title: State::record_timing
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::State::record_timing
anchors:
- sym:crates/myrn-store/src/state.rs#state::State::record_timing@12cd77a806acb0c8
---
# State::record_timing

Appends a timing and discards the oldest entries so at most 50 remain. It only changes memory and does not save, so the caller must call save to persist it.

## Connections

- **Location:** `crates/myrn-store/src/state.rs:68-72`
- **Signature:** `pub fn record_timing(&mut self, t: HookTiming)`
- **In:** [[file-crates-myrn-store-src-state-rs]]
- **Called by:** [[hook-handle]]
- **Tests:** `round_trip_and_missing_file`
