---
id: engine-session-agent
type: summary
status: unverified
title: Engine::session_agent
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::Engine::session_agent
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::Engine::session_agent@8643f16ba064aa56
map_version: '2'
---
# Engine::session_agent

Returns the agent name recorded in the episode that a note's source points to. Returns None when no agent can be found.

## Behaviour

- source None, or not starting with episode: -> None
- episode with only empty agent names -> None
- several episodes -> first non-empty agent is returned

## Terms

author agent, written by, episode agent, who wrote, agent name, session agent

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:56-64`
- **Signature:** `pub(crate) fn session_agent(&self, source: Option<&str>) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
- **Calls:** [[episodes-path-for]], [[episodes-read]], [[layout-episodes-dir]]
- **Called by:** [[engine-record]], [[engine-publish-plan]]
