---
id: signature-normalize
type: summary
status: unverified
title: signature::normalize
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::normalize
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::normalize@a0c5eab7a3a2b380
map_version: '2'
---
# signature::normalize

Entry point: trims and drops blank lines from raw error output, finds the error line, and returns its Signature with the message scrubbed, or None if there is no output or no error line.

## Behaviour

- empty or whitespace-only text -> None
- 'all good' and '3 passed' with no error marker -> None
- 'error[E0308]: mismatched types' among other lines -> kind 'E0308', message 'mismatched types'

## Terms

error output, stderr, compiler output, log text, error extraction, failure text, build output

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:68-82`
- **Signature:** `pub fn normalize(text: &str) -> Option<Signature>`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Calls:** [[signature-pick]], [[signature-scrub]]
- **Called by:** [[tests-sig]], [[engine-recall]]
- **Tests:** `generic_and_addresses`, `similarity`
