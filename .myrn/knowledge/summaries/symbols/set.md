---
id: set
type: summary
status: unverified
title: set
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#set
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#set@510bbe1811d30b41
---
# set

Builds a BTreeSet<String> from a slice of string literals, for comparison against ids output.

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:95-97`
- **Signature:** `fn set(v: &[&str]) -> BTreeSet<String>`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Called by:** [[check]]
- **Tests:** `gate0_rust`, `gate0_python`, `gate0_typescript`, `incremental_refresh_tracks_edits_and_deletes`
