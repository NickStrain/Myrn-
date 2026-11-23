---
id: file-crates-myrn-store-src-episodes-rs
type: summary
status: unverified
title: crates/myrn-store/src/episodes.rs
source: myrn summarize
about: file:crates/myrn-store/src/episodes.rs
anchors:
- file:crates/myrn-store/src/episodes.rs@cd902e38ca39209a
---
# crates/myrn-store/src/episodes.rs

Append-only JSONL episode logs for agent sessions in the private store, one Entry per line at episodes/<session>.jsonl. Main parts: the Entry record, file_name and path_for, which map a session id to a safe filename, append, read, and list.

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[episodes-entry]], [[episodes-file-name]], [[episodes-path-for]], [[episodes-append]], [[episodes-read]], [[episodes-list]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-engine-src-validity-gc-rs]], [[file-crates-myrn-engine-tests-history-rs]], [[file-crates-myrn-engine-tests-learning-rs]], [[file-crates-myrn-engine-tests-records-rs]], [[file-crates-myrn-engine-tests-validity-rs]], [[file-crates-myrn-store-src-lib-rs]]
