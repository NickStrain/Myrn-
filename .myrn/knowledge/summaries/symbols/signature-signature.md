---
id: signature-signature
type: summary
status: unverified
title: signature::Signature
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature@20689d35c8349679
map_version: '2'
---
# signature::Signature

Holds a normalized error as a kind and a scrubbed message. Derives Debug, Clone, PartialEq, Eq and Serialize.

## Behaviour

- kind 'E0308' and message 'mismatched types' -> two fields, serialized as JSON keys kind and message
- equality compares both fields exactly, case-sensitive
- empty message is allowed; text() then returns only the kind

## Terms

error key, failure fingerprint, error kind, error message, normalized error, error record

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:13-18`
- **Signature:** `pub struct Signature`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
