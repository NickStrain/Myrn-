---
id: sync-looks-like-sha
type: summary
status: unverified
title: sync::looks_like_sha
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::looks_like_sha
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::looks_like_sha@3794a121fa111407
map_version: '2'
---
# sync::looks_like_sha

Reports whether a string has the shape of a git commit prefix: 7 to 40 hex characters.

## Behaviour

- 'abc1234' gives true
- 'abc12' (6 characters) gives false
- 'xyz1234' gives false because of non-hex letters

## Terms

commit hash check, hex id, git sha, commit prefix

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:236-238`
- **Signature:** `fn looks_like_sha(t: &str) -> bool`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[resolver-resolve-sync]]
