---
id: distill-relationitem
type: summary
status: unverified
title: distill::RelationItem
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::RelationItem
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::RelationItem@125a100bdb0aa791
map_version: '2'
---
# distill::RelationItem

One typed link from a record: a relation name (`rel`) and the id it points to (`target`).

## Behaviour

- {"rel": "supersedes", "target": "fact-1a2b3c"} -> rel 'supersedes', target 'fact-1a2b3c'.
- Extra keys cause a deserialize error (deny_unknown_fields).
- No check that rel is in AGENT_RELATIONS happens in this struct.

## Terms

edge, link target, relation name, graph link

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:95-98`
- **Signature:** `pub struct RelationItem`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
