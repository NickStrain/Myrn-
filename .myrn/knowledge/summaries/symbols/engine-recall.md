---
id: engine-recall
type: summary
status: unverified
title: Engine::recall
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::Engine::recall
anchors:
- sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::Engine::recall@b43eb5f73188b6c6
---
# Engine::recall

Takes an error string and a limit, normalizes it to a signature, and returns up to limit non-superseded bug nodes whose stored signature has similarity at least 0.6, sorted by similarity then verified status then id; follows caused_by, located_in, and incoming fixes edges to fill causes and fixed_in.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs:35-98`
- **Signature:** `pub fn recall(&self, error: &str, limit: usize) -> Result<Vec<RecallHit>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-recall-rs]]
- **Calls:** [[dedupe-parse-signature]], [[signature-similarity]], [[signature-normalize]], [[node-location]], [[graph-nodes-where]], [[graph-neighbors]]
- **Called by:** [[engine-recall-text]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`
