---
id: dedupe-parse-signature
type: summary
status: unverified
title: dedupe::parse_signature
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::parse_signature
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::parse_signature@39d83aabbe45ee66
---
# dedupe::parse_signature

Splits a stored signature at the first ": " into kind and message; with no separator the whole string becomes kind and message is empty.

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:44-55`
- **Signature:** `pub fn parse_signature(s: &str) -> Signature`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Called by:** [[engine-recall]]
