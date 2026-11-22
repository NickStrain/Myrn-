---
id: copy-fixtures-rec
type: summary
status: unverified
title: copy_fixtures::rec
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#copy_fixtures::rec
anchors:
- sym:crates/myrn-engine/tests/notes.rs#copy_fixtures::rec@f4cf2a2484a96d47
---
# copy_fixtures::rec

Recursive helper that walks a fixture directory, applies the HEAD_SHA and BODY_HASH substitutions to each file's text, and writes it to the mirrored path under the destination.

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:75-89`
- **Signature:** `fn rec(from: &Path, to: &Path, head: &str, hash: &str)`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[copy-fixtures-rec]]
- **Called by:** [[copy-fixtures]], [[copy-fixtures-rec]]
