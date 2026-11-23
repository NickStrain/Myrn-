---
id: record-status
type: summary
status: unverified
title: record::Status
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::Status
anchors:
- sym:crates/myrn-model/src/record.rs#record::Status@6458d42ae02d8351
map_version: '2'
---
# record::Status

Enum of the four validity states an earned record can be in. Serializes and deserializes as snake_case strings.

## Behaviour

- Variants Unverified, Verified, Suspect, Superseded map to 'unverified', 'verified', 'suspect', 'superseded' in serde
- No default variant and no catch-all, so unknown strings fail to deserialize

## Terms

record validity, verification state, earned record state, unverified or verified, suspect record, superseded record

## Connections

- **Location:** `crates/myrn-model/src/record.rs:8-13`
- **Signature:** `pub enum Status`
- **In:** [[file-crates-myrn-model-src-record-rs]]
