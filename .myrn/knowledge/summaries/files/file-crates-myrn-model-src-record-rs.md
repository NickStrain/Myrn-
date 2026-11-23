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

Validity and visibility metadata for earned records and derived edges. Defines three serde enums: Status (record verification state), EdgeStatus (whether an edge was resolved or only inferred), and Visibility (private or shared), each with an as_str method that returns its snake_case name.

## Connections

- **Location:** `crates/myrn-model/src/record.rs`
- **In:** [[dir-crates-myrn-model-src]]
- **Defines:** [[record-status]], [[status-as-str]], [[status-parse]], [[record-edgestatus]], [[edgestatus-as-str]], [[record-visibility]], [[visibility-as-str]]
- **Imported by:** [[file-crates-myrn-model-src-lib-rs]]
