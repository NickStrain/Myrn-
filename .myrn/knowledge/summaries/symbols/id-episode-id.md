---
id: id-episode-id
type: summary
status: unverified
title: id::episode_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::episode_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::episode_id@134707234e599f2d
map_version: '2'
---
# id::episode_id

Builds an episode node id by prefixing a session identifier with `episode:`.

## Behaviour

- "s1" -> "episode:s1"
- Does not validate the session string

## Terms

session, session id, conversation, run id

## Connections

- **Location:** `crates/myrn-model/src/id.rs:37-39`
- **Signature:** `pub fn episode_id(session: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[episode-persist]], [[engine-current-session]]
