---
id: file-crates-myrn-engine-src-validity-gc-rs
type: summary
status: unverified
title: crates/myrn-engine/src/validity/gc.rs
source: myrn summarize
about: file:crates/myrn-engine/src/validity/gc.rs
anchors:
- file:crates/myrn-engine/src/validity/gc.rs@7836cb7108d53978
---
# crates/myrn-engine/src/validity/gc.rs

Garbage collection for `myrn gc`: removes duplicate note copies that match the indexed copy, deletes index rows whose targets are gone, and compacts session logs older than a cutoff. Main parts are `Engine::gc` (driver), `compact` (log line filter), `GcReport` (result), `now` (clock), and a unit test.

## Connections

- **Location:** `crates/myrn-engine/src/validity/gc.rs`
- **In:** [[dir-crates-myrn-engine-src-validity]]
- **Defines:** [[gc-gcreport]], [[engine-gc]], [[gc-compact]], [[gc-now]], [[tests-tool-gc]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-validity-mod-rs]]
