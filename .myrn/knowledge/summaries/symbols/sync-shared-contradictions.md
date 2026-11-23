---
id: sync-shared-contradictions
type: summary
status: unverified
title: sync::shared_contradictions
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::shared_contradictions
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::shared_contradictions@105efd1caa07bb97
map_version: '2'
---
# sync::shared_contradictions

Adds a 'contradicts' edge between two fact or decision records about the same code node when at least one is shared and their texts oppose each other, unless they are already linked.

## Behaviour

- superseded records are excluded
- two private records never produce an edge
- a pair already linked in either direction is skipped

## Terms

conflict detection, opposite claims, contradiction edges, disagreement check

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:551-601`
- **Signature:** `fn shared_contradictions(tx: &Transaction<'_>) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[claims-opposite]]
- **Called by:** [[sync-link]]
