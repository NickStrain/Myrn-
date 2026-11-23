---
id: dir-crates-myrn-engine-src-distill
type: summary
status: unverified
title: crates/myrn-engine/src/distill/
source: myrn summarize
about: module:crates/myrn-engine/src/distill
anchors:
- module:crates/myrn-engine/src/distill@3d1093f5db5c6959
---
# crates/myrn-engine/src/distill/

Validation and storage pipeline for distilled knowledge records (facts, decisions, bugs, symbol summaries) that agents write after a session. mod.rs defines the closed record-type, relation and evidence vocabularies and the RecordItem/RelationItem/RecordOutcome types; checks.rs validates fields and resolves references to code-graph nodes, commits or notes; contradiction.rs and dedupe.rs decide whether new text negates or duplicates stored notes; record.rs runs the full pipeline and writes the note file; signature.rs normalizes error output into comparable failure signatures for bug dedupe. mechanical.rs is a model-free episode summary that re-exports Episode from capture::episode.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-contradiction-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-mechanical-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-distill-signature-rs]]
