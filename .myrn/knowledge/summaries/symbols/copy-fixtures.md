---
id: copy-fixtures
type: summary
status: unverified
title: copy_fixtures
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#copy_fixtures
anchors:
- sym:crates/myrn-engine/tests/notes.rs#copy_fixtures@5cd38581d704f6ff
map_version: '2'
---
# copy_fixtures

Copies the note fixture tree into dest and replaces the HEAD_SHA and BODY_HASH placeholders in each text file with the given commit and hash.

## Behaviour

- head 'abcdef123...' -> HEAD_SHA becomes the first 7 characters
- body_hash is inserted verbatim for BODY_HASH
- head shorter than 7 characters or a non-UTF-8 file -> panics

## Terms

seed notes, placeholder substitution, fill placeholders, copy notes, stamp commit

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:74-91`
- **Signature:** `fn copy_fixtures(dest: &Path, head: &str, body_hash: &str)`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[fixtures]], [[copy-fixtures-rec]]
- **Called by:** [[indexed-repo]]
