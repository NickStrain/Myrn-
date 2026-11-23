---
id: state-record-timing
type: summary
status: unverified
title: State::record_timing
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::State::record_timing
anchors:
- sym:crates/myrn-store/src/state.rs#state::State::record_timing@12cd77a806acb0c8
map_version: '2'
---
# State::record_timing

Appends a HookTiming and trims the history so only the newest 50 remain, dropping the oldest first. It works in memory only and does not save to disk.

## Behaviour

- 0 timings stored, one added -> list holds 1 entry
- 51st timing added -> oldest entry removed, length stays 50
- 60 timings added in order 0..59 -> first remaining entry has millis 10, as the unit test checks

## Terms

trim timings, cap history, keep last 50, hook duration log, timing history, rolling window, drop oldest timing, record hook timing

## Connections

- **Location:** `crates/myrn-store/src/state.rs:71-75`
- **Signature:** `pub fn record_timing(&mut self, t: HookTiming)`
- **In:** [[file-crates-myrn-store-src-state-rs]]
- **Called by:** [[hook-handle]]
- **Tests:** `round_trip_and_missing_file`
