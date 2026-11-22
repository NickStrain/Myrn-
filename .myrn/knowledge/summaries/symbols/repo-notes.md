---
id: repo-notes
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#repo
anchors:
- sym:crates/myrn-engine/tests/notes.rs#repo@bacd0dc985d55f1a
---
# repo

Creates a temp git repo containing the PY sample files, commits them as 'first' with a fixed signature, then runs Engine::init with a home directory inside the same temp dir and returns the Repo.

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:45-67`
- **Signature:** `fn repo() -> Repo`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[engine-init]], [[write]]
- **Called by:** [[indexed-repo]]
- **Tests:** `write_note_writes_first_then_indexes_and_shared_wins`
