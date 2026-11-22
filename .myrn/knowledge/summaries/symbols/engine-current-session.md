---
id: engine-current-session
type: summary
status: unverified
title: Engine::current_session
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::current_session
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::current_session@f2c3116e99859ac4
---
# Engine::current_session

Returns the episode id for the given session name, or for the most recently listed episode when no name is given or the name is blank; returns None if listing fails or no episodes exist.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:412-426`
- **Signature:** `fn current_session(&self, given: Option<&str>) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[id-episode-id]], [[episodes-file-name]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[engine-record]]
