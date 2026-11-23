---
id: repo-notes
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#repo
anchors:
- sym:crates/myrn-engine/tests/notes.rs#repo@bacd0dc985d55f1a
map_version: '2'
---
# repo

Creates a fresh temp git repo containing the PY sample package, commits it, and initialises an Engine on it with its home under the temp dir.

## Behaviour

- no args -> repo with pkg/__init__.py, pkg/more.py, pkg/recipes.py and tests/test_recipes.py committed
- commit author 't' <t@example.invalid>, message 'first', no parent commit
- Engine home is tmp/home, so the user's real home is not touched

## Terms

temp repo, git init, sample repo, fresh repository, initial commit, engine setup

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:45-67`
- **Signature:** `fn repo() -> Repo`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[engine-init]], [[write]]
- **Called by:** [[indexed-repo]]
- **Tests:** `write_note_writes_first_then_indexes_and_shared_wins`
