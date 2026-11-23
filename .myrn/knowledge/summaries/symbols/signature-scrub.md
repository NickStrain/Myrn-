---
id: signature-scrub
type: summary
status: unverified
title: signature::scrub
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::scrub
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::scrub@24e8715b91bdb3a0
map_version: '2'
---
# signature::scrub

Replaces volatile parts of a message (timestamps, UUIDs, addresses, paths, long digit runs, hex ids, numbers) with fixed placeholders, then collapses whitespace.

## Behaviour

- 'token 3f9a2c1d8e expired at 2026-11-17T10:22:01Z' -> 'token ID expired at TIME'
- 'connect ECONNREFUSED 127.0.0.1:5432' -> 'connect ECONNREFUSED N.N:N'
- 'user_42' -> 'user_N'; 'utf8', 'i32', 'md5' keep their digits; 'and/or' -> PATH

## Terms

normalize message, redact, mask, placeholder, strip volatile parts, path stripping, sanitize, number replacement

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:153-193`
- **Signature:** `fn scrub(msg: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Called by:** [[signature-normalize]]
