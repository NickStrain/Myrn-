---
id: file-crates-myrn-engine-tests-validity-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/validity.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/validity.rs
anchors:
- file:crates/myrn-engine/tests/validity.rs@8a98e1240a3d4943
---
# crates/myrn-engine/tests/validity.rs

Integration tests for validity tracking, where records are checked against current code by symbol hash rather than commit. It holds fixture helpers (files, write, Repo, commit_all, repo, set, fact, status) and tests for the unverified, suspect and verified transitions, confirm, supersede, worktrees, shared notes and promotion.

## Connections

- **Location:** `crates/myrn-engine/tests/validity.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[files-validity]], [[write-validity]], [[repo-validity]], [[commit-all-validity]], [[repo-48c3]], [[status-validity]], [[set-validity]], [[fact-validity]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-store-src-episodes-rs]]
