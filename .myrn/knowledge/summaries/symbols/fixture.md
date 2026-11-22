---
id: fixture
type: summary
status: unverified
title: fixture
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#fixture
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#fixture@a470593ab824d686
---
# fixture

Returns the PathBuf to tests/fixtures/repos/gate0, resolved from CARGO_MANIFEST_DIR at compile time. This is the read-only source tree that setup copies from.

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:13-15`
- **Signature:** `fn fixture() -> PathBuf`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Called by:** [[setup]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`
