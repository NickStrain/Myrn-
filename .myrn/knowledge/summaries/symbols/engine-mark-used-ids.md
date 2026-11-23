---
id: engine-mark-used-ids
type: summary
status: unverified
title: Engine::mark_used_ids
source: myrn summarize
about: sym:crates/myrn-engine/src/learn.rs#learn::Engine::mark_used_ids
anchors:
- sym:crates/myrn-engine/src/learn.rs#learn::Engine::mark_used_ids@74226ca59295b35d
map_version: '2'
---
# Engine::mark_used_ids

Sets used = 1 on not-yet-used injections of the given node ids that happened within the last 2 hours, across any session. Returns the number of rows changed.

## Behaviour

- injection older than 7200 seconds -> not marked
- injection with a NULL timestamp (t) -> not marked, because coalesce turns it into 0
- empty ids -> 0 and no writes

## Terms

cited record, explain or recall use, citation marks used, recent injection, two hour window, use by node id, relation confirm or supersede

## Connections

- **Location:** `crates/myrn-engine/src/learn.rs:105-117`
- **Signature:** `pub fn mark_used_ids(&self, ids: &[String]) -> Result<usize>`
- **In:** [[file-crates-myrn-engine-src-learn-rs]]
- **Tests:** `opening_the_file_or_asking_about_it_counts_as_use`
