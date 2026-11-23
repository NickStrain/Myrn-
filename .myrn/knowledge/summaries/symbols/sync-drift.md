---
id: sync-drift
type: summary
status: unverified
title: sync::Drift
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Drift
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Drift@3bd3473795b4a70d
map_version: '2'
---
# sync::Drift

One anchor of a record whose hash no longer matches the current code, or whose code is gone.

## Behaviour

- current_hash is None when the anchored symbol no longer exists
- symbol_id is None when the anchor had no resolved symbol
- anchored_hash is the hash stored when the record was written

## Terms

stale anchor, anchor mismatch, drifted anchor, outdated anchor

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:695-702`
- **Signature:** `pub struct Drift`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
