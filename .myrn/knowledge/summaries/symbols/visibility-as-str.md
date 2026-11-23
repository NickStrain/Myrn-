---
id: visibility-as-str
type: summary
status: unverified
title: Visibility::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::Visibility::as_str
anchors:
- sym:crates/myrn-model/src/record.rs#record::Visibility::as_str@8f2017fdc460a48f
map_version: '2'
---
# Visibility::as_str

Returns the lowercase string name of a Visibility variant as a static str.

## Behaviour

- Visibility::Private -> 'private'
- Visibility::Shared -> 'shared'

## Terms

access label, scope string, sharing text, display name, serialized visibility value

## Connections

- **Location:** `crates/myrn-model/src/record.rs:63-68`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-record-rs]]
- **Called by:** [[engine-trace]], [[render-provenance]], [[render-safe-text]], [[viz-gnode-viz]], [[sync-sync]], [[sync-link]]
- **Tests:** `shared_records_that_disagree_are_flagged_for_a_person`
