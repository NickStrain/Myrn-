---
id: signature-pick
type: summary
status: unverified
title: signature::pick
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::pick
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::pick@f9321ae34da11719
---
# signature::pick

Scans lines for the error-naming form in priority order: rustc `error[EXXXX]`, tsc `error TSxxxx`, Rust `panicked at` (message inline or on the next line), then Python/JS exception class lines (reversed order when a Traceback header is present, and the class is reduced to its last dotted segment), then generic `error:`/`fatal:`/`failed` lines, and finally any line containing 'error' or 'FAILED'.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:85-150`
- **Signature:** `fn pick(lines: &[&str]) -> Option<(String, String)>`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Called by:** [[signature-normalize]]
