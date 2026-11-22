---
id: signature-words
type: summary
status: unverified
title: Signature::words
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::words
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::words@092a8be8e8201f1f
---
# Signature::words

Splits the message on non-alphanumeric characters (keeping `_`), lowercases, and drops tokens shorter than 2 chars or equal to `N`, `PATH`, or `ID`, for fuzzy matching.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:31-37`
- **Signature:** `pub fn words(&self) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Called by:** [[signature-similarity]]
