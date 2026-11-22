---
id: signature-scrub
type: summary
status: unverified
title: signature::scrub
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::scrub
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::scrub@24e8715b91bdb3a0
---
# signature::scrub

Applies 11 ordered regex rules that replace timestamps with TIME, UUIDs and long hex with ID, addresses with ADDR, paths with PATH, digit runs of 7+ with N, and remaining numbers with N (also `_N`/`-N` suffixes), then collapses whitespace; rule order matters, since paths are replaced before generic numbers.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:153-193`
- **Signature:** `fn scrub(msg: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Called by:** [[signature-normalize]]
