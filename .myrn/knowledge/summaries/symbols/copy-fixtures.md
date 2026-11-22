---
id: copy-fixtures
type: summary
status: unverified
title: copy_fixtures
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#copy_fixtures
anchors:
- sym:crates/myrn-engine/tests/notes.rs#copy_fixtures@5cd38581d704f6ff
---
# copy_fixtures

Copies the fixture notes tree into dest, replacing HEAD_SHA with the first 7 characters of head and BODY_HASH with the given hash string verbatim.

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:74-91`
- **Signature:** `fn copy_fixtures(dest: &Path, head: &str, body_hash: &str)`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[fixtures]], [[copy-fixtures-rec]]
- **Called by:** [[indexed-repo]]
