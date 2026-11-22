---
id: file-crates-myrn-model-src-record-rs
type: summary
status: unverified
title: crates/myrn-model/src/record.rs
source: myrn summarize
about: file:crates/myrn-model/src/record.rs
anchors:
- file:crates/myrn-model/src/record.rs@8b34b4278c50c578
---
# crates/myrn-model/src/record.rs

Defines the validity and provenance enums used for earned records and derived edges: Status (unverified, verified, suspect, superseded), EdgeStatus (resolved, inferred), and Visibility (private, shared). Each derives serde with snake_case names, and the enums that have as_str return the same lowercase strings.

## Connections

- **Location:** `crates/myrn-model/src/record.rs`
- **In:** [[dir-crates-myrn-model-src]]
- **Defines:** [[record-status]], [[status-as-str]], [[status-parse]], [[record-edgestatus]], [[edgestatus-as-str]], [[record-visibility]], [[visibility-as-str]]
- **Imported by:** [[file-crates-myrn-model-src-lib-rs]]
