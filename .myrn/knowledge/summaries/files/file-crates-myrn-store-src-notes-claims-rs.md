---
id: file-crates-myrn-store-src-notes-claims-rs
type: summary
status: unverified
title: crates/myrn-store/src/notes/claims.rs
source: myrn summarize
about: file:crates/myrn-store/src/notes/claims.rs
anchors:
- file:crates/myrn-store/src/notes/claims.rs@1dd37f3c50ae709d
---
# crates/myrn-store/src/notes/claims.rs

Model-free check for whether two statements make opposite claims about the same subject: nearly identical wording where one side negates the other. Used when a record is stored and when shared records are linked. Main parts are `words`, which tokenizes text into a content-word set and a negation count, and `opposite`, which compares a pair of statements.

## Connections

- **Location:** `crates/myrn-store/src/notes/claims.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[claims-words]], [[claims-opposite]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-contradiction-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-sync-rs]]
