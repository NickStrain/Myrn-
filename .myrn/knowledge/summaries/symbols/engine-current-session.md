---
id: engine-current-session
type: summary
status: unverified
title: Engine::current_session
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::current_session
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::current_session@f2c3116e99859ac4
map_version: '2'
---
# Engine::current_session

Returns the episode id for a given session name, or for the first listed episode when no non-blank session is given. Returns None when there is no episode to use.

## Behaviour

- session 'abc' -> file name with the .jsonl suffix removed, then converted to an episode id
- blank or missing session -> first entry of episodes::list
- no episodes or a listing error -> None

## Terms

episode id, latest session, default session, provenance source, session lookup

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:423-437`
- **Signature:** `fn current_session(&self, given: Option<&str>) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[id-episode-id]], [[episodes-file-name]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[engine-record]]
