---
id: fixtures
type: summary
status: unverified
title: fixtures
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#fixtures
anchors:
- sym:crates/myrn-engine/tests/notes.rs#fixtures@f7624142a1d1ae95
map_version: '2'
---
# fixtures

Returns the path to the note fixture folder tests/fixtures/notes, located relative to the crate manifest directory.

## Behaviour

- no args -> path ending in tests/fixtures/notes at the workspace root
- does not check that the folder exists
- no filesystem access

## Terms

fixture path, notes fixtures, test data, sample notes

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:69-71`
- **Signature:** `fn fixtures() -> PathBuf`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Called by:** [[copy-fixtures]]
