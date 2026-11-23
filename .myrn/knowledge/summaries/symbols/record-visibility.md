---
id: record-visibility
type: summary
status: unverified
title: record::Visibility
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::Visibility
anchors:
- sym:crates/myrn-model/src/record.rs#record::Visibility@08d43f09918fb0fe
map_version: '2'
---
# record::Visibility

Enum with two sharing scopes, Private and Shared. Serializes and deserializes as snake_case strings.

## Behaviour

- Private serializes as 'private'; Shared serializes as 'shared'
- No default variant, so unknown strings fail to deserialize

## Terms

access level, sharing scope, private or shared, record access, visibility setting, scope

## Connections

- **Location:** `crates/myrn-model/src/record.rs:57-60`
- **Signature:** `pub enum Visibility`
- **In:** [[file-crates-myrn-model-src-record-rs]]
