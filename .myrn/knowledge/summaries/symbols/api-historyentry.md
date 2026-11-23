---
id: api-historyentry
type: summary
status: unverified
title: api::HistoryEntry
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::HistoryEntry
anchors:
- sym:crates/myrn-engine/src/api.rs#api::HistoryEntry@696a51664e0348bf
map_version: '2'
---
# api::HistoryEntry

One commit that touched a file or symbol: sha, summary, author, unix time, change kind and optional change label. Parent shas are kept for ordering but not serialised.

## Behaviour

- change is the git change kind for file queries and modified for symbol queries
- parents is skipped in serialisation and used only to order commits made in the same second

## Terms

commit entry, changelog, change kind, author, parent commits, git log

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:84-92`
- **Signature:** `pub struct HistoryEntry`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
