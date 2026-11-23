---
id: state-save
type: summary
status: unverified
title: State::save
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::State::save
anchors:
- sym:crates/myrn-store/src/state.rs#state::State::save@17fb0ef00a61ecee
map_version: '2'
---
# State::save

Writes the State as pretty-printed JSON to the given path. It writes to a temp file and renames it into place, so readers never see a half-written file.

## Behaviour

- parent directory missing -> created with create_dir_all before writing
- temp file is path with extension replaced by json.<process id>.tmp, e.g. state.json -> state.json.4242.tmp, so concurrent processes do not share a temp file
- rename failure or serialization failure -> Err, and the temp file is left behind on rename failure

## Terms

atomic write, write state file, temp file rename, persist state, safe write, concurrent writes, save state, crash safe write

## Connections

- **Location:** `crates/myrn-store/src/state.rs:61-69`
- **Signature:** `pub fn save(&self, path: &Path) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-state-rs]]
- **Called by:** [[engine-save-state]]
- **Tests:** `round_trip_and_missing_file`
