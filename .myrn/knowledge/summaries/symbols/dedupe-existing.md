---
id: dedupe-existing
type: summary
status: unverified
title: dedupe::Existing
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::Existing
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::Existing@b89dcea5d73c9227
map_version: '2'
---
# dedupe::Existing

Pairs a notes-table key with the parsed Note it belongs to.

## Behaviour

- key -> the path column of the notes row
- note -> the data column deserialized as a Note
- returned by every lookup in this file

## Terms

lookup result, stored note, note record, row key, path key

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:23-26`
- **Signature:** `pub struct Existing`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
