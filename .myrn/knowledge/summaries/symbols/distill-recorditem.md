---
id: distill-recorditem
type: summary
status: unverified
title: distill::RecordItem
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordItem
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordItem@84d29feb871cc345
---
# distill::RecordItem

Deserialized form of one record item: the JSON key 'type' maps to kind, and only type and body are required. Unknown fields are rejected, and about, located_in and verified_by accept either a single string or a list.

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:47-91`
- **Signature:** `pub struct RecordItem`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
