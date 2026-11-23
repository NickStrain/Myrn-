---
id: file-crates-myrn-engine-src-distill-contradiction-rs
type: summary
status: unverified
title: crates/myrn-engine/src/distill/contradiction.rs
source: myrn summarize
about: file:crates/myrn-engine/src/distill/contradiction.rs
anchors:
- file:crates/myrn-engine/src/distill/contradiction.rs@2938c68031464b2f
---
# crates/myrn-engine/src/distill/contradiction.rs

Contradiction detection for fact and decision text. The only item in this excerpt is `find`, which looks up existing facts and decisions attached to target nodes by `about` edges and keeps those that the new body text negates, using the store's `opposite` check.

## Connections

- **Location:** `crates/myrn-engine/src/distill/contradiction.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[contradiction-find]]
- **Imports:** [[file-crates-myrn-store-src-notes-claims-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]]
