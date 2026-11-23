---
id: dedupe-parse-signature
type: summary
status: unverified
title: dedupe::parse_signature
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::parse_signature
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::parse_signature@39d83aabbe45ee66
map_version: '2'
---
# dedupe::parse_signature

Splits a stored signature string at its first colon-space into an error kind and an error message.

## Behaviour

- 'TypeError: unhashable type' -> kind 'TypeError', message 'unhashable type'
- 'a: b: c' -> kind 'a', message 'b: c' (splits at the first ': ' only)
- 'Timeout' with no ': ' -> kind 'Timeout', message empty string

## Terms

error kind, error message, error type, colon split, stored text, decode

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:44-55`
- **Signature:** `pub fn parse_signature(s: &str) -> Signature`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Called by:** [[engine-recall]]
