---
id: engine-recall
type: summary
status: unverified
title: Engine::recall
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::Engine::recall
anchors:
- sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::Engine::recall@473889ebbe24b1d0
map_version: '2'
---
# Engine::recall

Scans non-superseded bug nodes, keeps those whose stored signature is at least 0.6 similar to the normalized error, attaches causes and fixing commits, and returns them sorted and truncated to limit.

## Behaviour

- error that normalizes to no signature -> empty Vec
- bug whose extra JSON lacks 'signature' is skipped; similarity below 0.6 is skipped
- equal similarity: status 'verified' sorts first, then id ascending; limit 0 -> empty Vec

## Terms

error lookup, signature match, similarity threshold, known bug search, superseded excluded, root cause, ranking order, fixing commits

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs:38-103`
- **Signature:** `pub fn recall(&self, error: &str, limit: usize) -> Result<Vec<RecallHit>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-recall-rs]]
- **Calls:** [[dedupe-parse-signature]], [[signature-similarity]], [[signature-normalize]], [[render-provenance]], [[render-safe-text]], [[node-location]], [[graph-nodes-where]], [[graph-neighbors]]
- **Called by:** [[engine-recall-text]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`
