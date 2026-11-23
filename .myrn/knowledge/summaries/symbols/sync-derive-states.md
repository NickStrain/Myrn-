---
id: sync-derive-states
type: summary
status: unverified
title: sync::derive_states
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::derive_states
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::derive_states@b4b1a53269c1295c
map_version: '2'
---
# sync::derive_states

Recomputes record status in the nodes table: superseded when a supersedes edge points at it, suspect when an anchor's body hash no longer matches the code.

## Behaviour

- a record already superseded is not changed to suspect
- an anchor whose symbol is missing or has no body hash makes the record suspect
- status is recomputed from current hashes each run, so returning to matching code clears suspect

## Terms

superseded, suspect, status recompute, stale record, anchor mismatch

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:676-691`
- **Signature:** `fn derive_states(tx: &Transaction<'_>) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[sync-link]]
