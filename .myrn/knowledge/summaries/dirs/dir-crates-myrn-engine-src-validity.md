---
id: dir-crates-myrn-engine-src-validity
type: summary
status: unverified
title: crates/myrn-engine/src/validity/
source: myrn summarize
about: module:crates/myrn-engine/src/validity
anchors:
- module:crates/myrn-engine/src/validity@116adfd88e43868f
---
# crates/myrn-engine/src/validity/

Validity tracking for myrn-engine: records are kept valid by hash anchors, which compare stored anchor body hashes against current ones and mark records suspect on mismatch (anchors.rs, currently doc-only). states.rs is intended to hold the record state transitions (unverified to verified, suspect, or superseded, plus confirm and supersede), and gc.rs is intended for merging duplicate records, pruning index entries for deleted symbols, and compacting episodes, never deleting earned records unless explicitly asked. All three files currently contain only module doc comments, and mod.rs only declares the anchors, gc, and states submodules.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-validity-anchors-rs]], [[file-crates-myrn-engine-src-validity-gc-rs]], [[file-crates-myrn-engine-src-validity-mod-rs]], [[file-crates-myrn-engine-src-validity-states-rs]]
