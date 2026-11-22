---
id: file-crates-myrn-engine-src-validity-gc-rs
type: summary
status: unverified
title: crates/myrn-engine/src/validity/gc.rs
source: myrn summarize
about: file:crates/myrn-engine/src/validity/gc.rs
anchors:
- file:crates/myrn-engine/src/validity/gc.rs@8e54203acb1f2204
---
# crates/myrn-engine/src/validity/gc.rs

Garbage-collection module for the validity subsystem: merges duplicate records, removes index entries that point at deleted symbols, and compacts episodes. Per its module doc, it must never delete earned records unless explicitly asked; the source shown contains only this doc comment, with no functions or types defined yet.

## Connections

- **Location:** `crates/myrn-engine/src/validity/gc.rs`
- **In:** [[dir-crates-myrn-engine-src-validity]]
- **Imported by:** [[file-crates-myrn-engine-src-validity-mod-rs]]
