---
id: repo-learning
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/learning.rs#repo
anchors:
- sym:crates/myrn-engine/tests/learning.rs#repo@b605ce3aabf68069
map_version: '2'
---
# repo

Creates a temp directory with a git repo holding auth/issue.py (issue_token) and auth/check.py (check_token), commits them as a root commit, and initialises an Engine at that repo. Returns the TempDir guard with the Engine.

## Behaviour

- called -> (TempDir, Engine) where the engine root is tmp/repo
- TempDir dropped -> the whole temp directory is deleted, so the caller must keep it bound
- commit has no parents and message "first", author t <t@example.invalid>

## Terms

fixture, temp repo, test project, git commit, engine setup, sample project

## Connections

- **Location:** `crates/myrn-engine/tests/learning.rs:19-41`
- **Signature:** `fn repo() -> (tempfile::TempDir, Engine)`
- **In:** [[file-crates-myrn-engine-tests-learning-rs]]
- **Calls:** [[engine-init]], [[write-learning]]
- **Tests:** `opening_the_file_or_asking_about_it_counts_as_use`, `sessions_attach_their_outcome`, `learned_ranking_moves_items_by_past_use_and_fixed_ignores_it`, `without_embeddings_nothing_changes`
