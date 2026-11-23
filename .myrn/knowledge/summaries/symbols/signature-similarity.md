---
id: signature-similarity
type: summary
status: unverified
title: Signature::similarity
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::similarity
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::similarity@d0c8f34c82a8f5a8
map_version: '2'
---
# Signature::similarity

Scores how alike two signatures are from 0 to 1: 0 for different kinds, 1 for identical messages, otherwise the Jaccard overlap of their word sets.

## Behaviour

- kinds 'TypeError' and 'typeerror' with the same message -> 1.0 (kind compared ignoring ASCII case)
- kinds 'TypeError' and 'ValueError' -> 0.0 even when messages match
- messages identical but empty -> 1.0; one message empty and the other not -> 0.0

## Terms

match score, jaccard, overlap, fuzzy match, word overlap, similarity ratio, kind mismatch

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:40-53`
- **Signature:** `pub fn similarity(&self, other: &Signature) -> f64`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Calls:** [[signature-words]]
- **Called by:** [[dedupe-find-bug]], [[engine-recall]]
- **Tests:** `similarity`
