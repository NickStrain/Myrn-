---
id: dir-crates-myrn-engine-src-distill
type: summary
status: unverified
title: crates/myrn-engine/src/distill/
source: myrn summarize
about: module:crates/myrn-engine/src/distill
anchors:
- module:crates/myrn-engine/src/distill@a8329664312c33c1
---
# crates/myrn-engine/src/distill/

Distills agent-reported bugs, decisions, facts, workflows, and summaries into stored notes. mod.rs defines the record schema, RECORD_TYPES/AGENT_RELATIONS/EVIDENCE_KINDS vocabularies, and the outcome returned to the agent; record.rs validates a record, resolves references via checks.rs, merges it with a match from dedupe.rs, writes the note, and syncs the index. contradiction.rs adds contradicts edges using a lexical negation-parity check (SAME_CLAIM 0.6), mechanical.rs re-exports Episode::summary as a model-free fallback, and signature.rs normalizes error output into stable (kind, message) Signatures for matching.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-contradiction-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-mechanical-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-distill-signature-rs]]
