---
id: distill-recorditem
type: summary
status: unverified
title: distill::RecordItem
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordItem
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordItem@84d29feb871cc345
map_version: '2'
---
# distill::RecordItem

Deserialized input for one `record` tool call: a decision, bug, workflow, fact or summary, with its evidence and type-specific fields.

## Behaviour

- JSON key 'type' maps to field kind; any unknown key such as 'depends_on' fails to deserialize.
- about, located_in and verified_by accept a single string or an array; a null or absent value gives an empty Vec.
- evidence, steps and relations default to empty; title, error, fix, fixed_in and session default to None.

## Terms

tool argument, distilled note, evidence list, root cause, fix commit, workflow steps, typed link, session id

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:47-91`
- **Signature:** `pub struct RecordItem`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
