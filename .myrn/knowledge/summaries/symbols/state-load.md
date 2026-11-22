---
id: state-load
type: summary
status: unverified
title: State::load
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::State::load
anchors:
- sym:crates/myrn-store/src/state.rs#state::State::load@c67c8487c2855268
---
# State::load

Reads and parses JSON from path, returning State::default() if the file does not exist. Other I/O errors propagate, and a parse failure is returned with the path in its error context.

## Connections

- **Location:** `crates/myrn-store/src/state.rs:47-55`
- **Signature:** `pub fn load(path: &Path) -> Result<State>`
- **In:** [[file-crates-myrn-store-src-state-rs]]
- **Called by:** [[engine-state]]
- **Tests:** `round_trip_and_missing_file`
