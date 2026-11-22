---
id: signature-normalize
type: summary
status: unverified
title: signature::normalize
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::normalize
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::normalize@a0c5eab7a3a2b380
---
# signature::normalize

Entry point: trims and drops blank lines from the input, calls `pick` to find the error line, and scrubs its message; returns None when the input is empty or no error line is found.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:68-82`
- **Signature:** `pub fn normalize(text: &str) -> Option<Signature>`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Calls:** [[signature-pick]], [[signature-scrub]]
- **Called by:** [[tests-sig]], [[engine-recall]]
- **Tests:** `generic_and_addresses`, `similarity`
