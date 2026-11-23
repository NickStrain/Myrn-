---
id: file-crates-myrn-engine-src-learn-rs
type: summary
status: unverified
title: crates/myrn-engine/src/learn.rs
source: myrn summarize
about: file:crates/myrn-engine/src/learn.rs
anchors:
- file:crates/myrn-engine/src/learn.rs@32196565abcc59a8
---
# crates/myrn-engine/src/learn.rs

The learning loop for ranking. It stores which injected items the agent later used and how the session ended, then turns that history into ranking factors (Usage::factor, freshness) and exposes the Engine methods that update and read the usage table.

## Connections

- **Location:** `crates/myrn-engine/src/learn.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[learn-usage]], [[usage-factor]], [[learn-usage-of]], [[learn-freshness]], [[engine-mark-used-files]], [[engine-mark-used-ids]], [[engine-attach-outcome]], [[engine-usage]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-config-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-lib-rs]]
