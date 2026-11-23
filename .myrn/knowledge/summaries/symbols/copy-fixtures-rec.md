---
id: copy-fixtures-rec
type: summary
status: unverified
title: copy_fixtures::rec
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#copy_fixtures::rec
anchors:
- sym:crates/myrn-engine/tests/notes.rs#copy_fixtures::rec@f4cf2a2484a96d47
map_version: '2'
---
# copy_fixtures::rec

Recursively walks the source folder, copying each text file to the destination with the placeholders replaced and recreating subfolders.

## Behaviour

- subdirectory -> recurses into it and creates the matching destination folder
- file -> read as text, replace HEAD_SHA with head[..7] and BODY_HASH with hash, then write
- file names are copied unchanged, only file contents are substituted

## Terms

recursive copy, walk directory, copy tree, substitute text

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:75-89`
- **Signature:** `fn rec(from: &Path, to: &Path, head: &str, hash: &str)`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[copy-fixtures-rec]]
- **Called by:** [[copy-fixtures]], [[copy-fixtures-rec]]
