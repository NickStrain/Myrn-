---
id: knowledge-forgotten
type: summary
status: unverified
title: knowledge::Forgotten
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::Forgotten
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::Forgotten@cbb4977e81d04ec0
map_version: '2'
---
# knowledge::Forgotten

Result of `forget_plan` or `forget`: the record ids to delete, the note files backing them, and other records whose links to them are removed.

## Behaviour

- ids always begins with the requested id, followed by root causes only it used
- unlinked_from excludes ids already in the plan and duplicates
- files lists every copy of each id's note file, shared and private

## Terms

deletion, removal, forget plan, purge, dependent records, root cause

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:100-105`
- **Signature:** `pub struct Forgotten`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
