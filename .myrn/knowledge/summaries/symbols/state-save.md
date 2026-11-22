---
id: state-save
type: summary
status: unverified
title: State::save
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::State::save
anchors:
- sym:crates/myrn-store/src/state.rs#state::State::save@17fb0ef00a61ecee
---
# State::save

Creates the parent directory, writes pretty JSON to a temp file named with the process id, then renames it over path so readers never see a partial file. Concurrent hook processes do not clobber each other's temp files.

## Connections

- **Location:** `crates/myrn-store/src/state.rs:58-66`
- **Signature:** `pub fn save(&self, path: &Path) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-state-rs]]
- **Called by:** [[engine-save-state]]
- **Tests:** `round_trip_and_missing_file`
