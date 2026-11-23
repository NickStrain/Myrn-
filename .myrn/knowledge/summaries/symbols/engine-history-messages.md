---
id: engine-history-messages
type: summary
status: unverified
title: Engine::history_messages
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::history_messages
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::history_messages@964a0833c2ee6d2f
map_version: '2'
---
# Engine::history_messages

Reads commit id and message from the current branch's history, newest first, up to the configured depth.

## Behaviour

- Repository cannot be opened, or HEAD is missing -> empty vec
- Limit is index.history_depth, and at least 1 even when set to 0

## Terms

git log, commit history, branch log, commit messages

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:319-334`
- **Signature:** `fn history_messages(&self) -> Result<Vec<(String, String)>>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Calls:** [[engine-root]]
- **Called by:** [[engine-promote]]
