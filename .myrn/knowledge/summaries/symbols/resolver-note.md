---
id: resolver-note
type: summary
status: unverified
title: Resolver::note
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Resolver::note
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Resolver::note@7af51700cdab19b6
map_version: '2'
---
# Resolver::note

Returns the note id matching a given text by lowercase comparison.

## Behaviour

- 'FOO' finds a note whose id is 'foo' and returns its id as stored
- unknown text gives None
- matching ignores case only, not punctuation

## Terms

note lookup, case-insensitive id, note id match

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:241-243`
- **Signature:** `fn note(&self, t: &str) -> Option<String>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[resolver-resolve-sync]], [[sync-link]]
