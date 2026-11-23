---
id: engine-attach-outcome
type: summary
status: unverified
title: Engine::attach_outcome
source: myrn summarize
about: sym:crates/myrn-engine/src/learn.rs#learn::Engine::attach_outcome
anchors:
- sym:crates/myrn-engine/src/learn.rs#learn::Engine::attach_outcome@786bd4ca3f41b346
map_version: '2'
---
# Engine::attach_outcome

Writes the session's episode outcome into the outcome column of every usage row for that session, replacing any earlier value. Does nothing when the session has no episode log.

## Behaviour

- no episode log for the session -> Ok(()) with no writes
- episode exists -> every usage row for the session gets the outcome, including rows with injected = 0
- a second call overwrites the earlier outcome

## Terms

session outcome, turn end, episode result, outcome recording, session finish, tests passed outcome, committed outcome

## Connections

- **Location:** `crates/myrn-engine/src/learn.rs:120-129`
- **Signature:** `pub fn attach_outcome(&self, session: &str) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-learn-rs]]
- **Calls:** [[engine-episode-log]], [[episode-outcome]]
- **Called by:** [[hook-handle]]
- **Tests:** `sessions_attach_their_outcome`
