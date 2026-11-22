---
id: signature-similarity
type: summary
status: unverified
title: Signature::similarity
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::similarity
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::similarity@d0c8f34c82a8f5a8
---
# Signature::similarity

Returns a 0.0 to 1.0 Jaccard score over word sets; returns 0.0 when kinds differ (case-insensitive) or either word set is empty, and 1.0 on exact message equality.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:40-53`
- **Signature:** `pub fn similarity(&self, other: &Signature) -> f64`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Calls:** [[signature-words]]
- **Called by:** [[dedupe-find-bug]], [[engine-recall]]
- **Tests:** `similarity`
