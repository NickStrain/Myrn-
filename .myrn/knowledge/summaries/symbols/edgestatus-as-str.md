---
id: edgestatus-as-str
type: summary
status: unverified
title: EdgeStatus::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::EdgeStatus::as_str
anchors:
- sym:crates/myrn-model/src/record.rs#record::EdgeStatus::as_str@55e20b68329695e9
map_version: '2'
---
# EdgeStatus::as_str

Returns the lowercase string name of an EdgeStatus variant as a static str.

## Behaviour

- EdgeStatus::Resolved -> 'resolved'
- EdgeStatus::Inferred -> 'inferred'

## Terms

edge label, confidence string, provenance text, display name, serialized edge value

## Connections

- **Location:** `crates/myrn-model/src/record.rs:47-52`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-record-rs]]
