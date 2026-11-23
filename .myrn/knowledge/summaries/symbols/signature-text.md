---
id: signature-text
type: summary
status: unverified
title: Signature::text
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::text
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::text@04b1daf3d1745105
map_version: '2'
---
# Signature::text

Formats the signature as 'kind: message' for storage and recall, or as the bare kind when the message is empty.

## Behaviour

- kind 'TypeError', message empty -> 'TypeError' (no colon)
- kind 'E0308', message 'mismatched types' -> 'E0308: mismatched types'
- this string is the form stored on bug notes and compared on recall

## Terms

stored form, display string, key string, one-line form, bug note text, recall key, string form

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:22-28`
- **Signature:** `pub fn text(&self) -> String`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
