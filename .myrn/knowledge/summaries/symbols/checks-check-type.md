---
id: checks-check-type
type: summary
status: unverified
title: checks::check_type
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::check_type
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::check_type@0d9fb2398cb3446f
map_version: '2'
---
# checks::check_type

Returns an error message if the record type is not in RECORD_TYPES, otherwise None.

## Behaviour

- "finding" when finding is listed -> None
- "Finding" -> Some message, because the check is case-sensitive
- message lists the allowed types and echoes the bad value

## Terms

record type, closed vocabulary, type validation, allowed types

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:32-39`
- **Signature:** `pub fn check_type(kind: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Called by:** [[engine-record]]
