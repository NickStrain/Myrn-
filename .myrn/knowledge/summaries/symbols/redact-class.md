---
id: redact-class
type: summary
status: unverified
title: redact::Class
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::Class
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::Class@4cef20f197cdf2c7
map_version: '2'
---
# redact::Class

Enum for the character category used by looks_random to count class switches and digits. It is private to this module.

## Behaviour

- Variants are Lower, Upper, Digit and Other
- Used only inside looks_random through class()

## Terms

character category, char bucket, case and digit kind, class switch

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:188-193`
- **Signature:** `enum Class`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
