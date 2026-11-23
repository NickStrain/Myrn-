---
id: engine-summary-outdated
type: summary
status: unverified
title: Engine::summary_outdated
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::summary_outdated
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::summary_outdated@d97b040996ed4378
map_version: '2'
---
# Engine::summary_outdated

Reports whether a node's summary was written for a body hash different from the current one. It compares the current body hash with the stored hashes of summary notes anchored to the node.

## Behaviour

- no summary or no body hash on the node -> false
- no summary anchors with a body hash -> false, so unanchored summaries are not outdated
- current body hash starts with some stored summary hash -> false; no match -> true

## Terms

stale, outdated, body changed, hash mismatch, summary drift

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:554-566`
- **Signature:** `pub fn summary_outdated(&self, n: &graph::Node) -> Result<bool>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Called by:** [[engine-symbol-card]], [[engine-summary-is-current]]
