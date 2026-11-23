---
id: sync-set-local-verified
type: summary
status: unverified
title: sync::set_local_verified
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::set_local_verified
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::set_local_verified@0d81ee276fd819dc
map_version: '2'
---
# sync::set_local_verified

Records that this clone has verified a shared record as its body reads now, tied to the body hash.

## Behaviour

- writes or replaces the local_verified row for note_id
- a later body edit changes the body hash, so verification no longer matches
- time t is seconds since the Unix epoch, or 0 if the clock is before it

## Terms

verify record, local verification, mark verified, trust record

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:652-667`
- **Signature:** `pub fn set_local_verified( conn: &Connection, note_id: &str, body: &str, reason: &str, ) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[sync-body-hash]]
- **Called by:** [[engine-publish]], [[engine-promote]]
