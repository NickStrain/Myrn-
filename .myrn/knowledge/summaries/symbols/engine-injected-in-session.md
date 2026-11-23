---
id: engine-injected-in-session
type: summary
status: unverified
title: Engine::injected_in_session
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::injected_in_session
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::injected_in_session@5e6bc8816e6879de
map_version: '2'
---
# Engine::injected_in_session

Returns the set of note ids already injected into a session.

## Behaviour

- Reads distinct note ids from usage rows where injected is 1
- Unknown session -> empty set

## Terms

session history, already sent, duplicate injection, injected notes

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:684-693`
- **Signature:** `pub fn injected_in_session(&self, session: &str) -> Result<HashSet<String>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[hook-handle]]
