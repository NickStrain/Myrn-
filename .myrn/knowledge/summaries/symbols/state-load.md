---
id: state-load
type: summary
status: unverified
title: State::load
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::State::load
anchors:
- sym:crates/myrn-store/src/state.rs#state::State::load@c67c8487c2855268
map_version: '2'
---
# State::load

Reads and parses state.json from the given path into a State. A missing file yields the default State instead of an error.

## Behaviour

- path does not exist -> Ok(State::default()), no file is created
- file contains invalid JSON -> Err with context "reading <path>"
- other read errors such as permission denied -> Err returned unchanged

## Terms

read state, load state file, missing file default, deserialize state, parse state json, first run state, read config

## Connections

- **Location:** `crates/myrn-store/src/state.rs:50-58`
- **Signature:** `pub fn load(path: &Path) -> Result<State>`
- **In:** [[file-crates-myrn-store-src-state-rs]]
- **Called by:** [[engine-state]]
- **Tests:** `round_trip_and_missing_file`
