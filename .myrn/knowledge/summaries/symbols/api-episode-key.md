---
id: api-episode-key
type: summary
status: unverified
title: api::episode_key
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::episode_key
anchors:
- sym:crates/myrn-engine/src/api.rs#api::episode_key@f1a80cade38ca1e9
map_version: '2'
---
# api::episode_key

Returns the file stem of an episode log path, used as its session key.

## Behaviour

- A path with no file stem -> empty string

## Terms

session key, file stem, episode name, log name

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1382-1386`
- **Signature:** `fn episode_key(path: &Path) -> String`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-capture]]
