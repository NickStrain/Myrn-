---
id: api-episode-key
type: summary
status: unverified
title: api::episode_key
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::episode_key
anchors:
- sym:crates/myrn-engine/src/api.rs#api::episode_key@f1a80cade38ca1e9
---
# api::episode_key

Returns the file stem of an episode log path to use as its key, or an empty string when there is none.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1306-1310`
- **Signature:** `fn episode_key(path: &Path) -> String`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-capture]]
