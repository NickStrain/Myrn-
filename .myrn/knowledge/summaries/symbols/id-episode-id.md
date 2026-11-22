---
id: id-episode-id
type: summary
status: unverified
title: id::episode_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::episode_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::episode_id@134707234e599f2d
---
# id::episode_id

Takes a session identifier and returns `episode:<session>` as given.

## Connections

- **Location:** `crates/myrn-model/src/id.rs:37-39`
- **Signature:** `pub fn episode_id(session: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[episode-persist]], [[engine-current-session]]
