---
id: dir-crates-myrn-engine-src-validity
type: summary
status: unverified
title: crates/myrn-engine/src/validity/
source: myrn summarize
about: module:crates/myrn-engine/src/validity
anchors:
- module:crates/myrn-engine/src/validity@fe564b4fe4229fed
---
# crates/myrn-engine/src/validity/

Validity tracking decides whether earned notes still hold by checking hash anchors against current code. States.rs derives suspect and superseded states, earns verified status, and provides confirm and supersede operations without deleting anything; anchors.rs builds the suspect notice shown when an agent works in a described file; gc.rs runs `myrn gc` to remove duplicate note copies, stale index rows, and old session logs. mod.rs only declares the anchors, gc, and states submodules.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-validity-anchors-rs]], [[file-crates-myrn-engine-src-validity-gc-rs]], [[file-crates-myrn-engine-src-validity-mod-rs]], [[file-crates-myrn-engine-src-validity-states-rs]]
