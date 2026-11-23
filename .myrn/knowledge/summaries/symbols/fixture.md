---
id: fixture
type: summary
status: unverified
title: fixture
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#fixture
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#fixture@a470593ab824d686
map_version: '2'
---
# fixture

Returns the path to the gate0 fixture repo, resolved from the crate manifest dir up two levels.

## Behaviour

- no args -> PathBuf ending in tests/fixtures/repos/gate0
- path is built at call time from CARGO_MANIFEST_DIR, so it depends on the crate location

## Terms

fixture path, test repo location, gate0 repo

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:13-15`
- **Signature:** `fn fixture() -> PathBuf`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Called by:** [[setup]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`
