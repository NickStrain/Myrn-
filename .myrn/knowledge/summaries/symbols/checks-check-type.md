---
id: checks-check-type
type: summary
status: unverified
title: checks::check_type
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::check_type
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::check_type@0d9fb2398cb3446f
---
# checks::check_type

Returns a error message listing RECORD_TYPES if the kind is not in that closed set, otherwise None.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:32-39`
- **Signature:** `pub fn check_type(kind: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Called by:** [[engine-record]]
