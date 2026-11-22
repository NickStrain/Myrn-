---
id: engine-injected-in-session
type: summary
status: unverified
title: Engine::injected_in_session
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::injected_in_session
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::injected_in_session@5e6bc8816e6879de
---
# Engine::injected_in_session

Returns the note ids already injected into the given session, so later prompts do not receive them again.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:655-664`
- **Signature:** `pub fn injected_in_session(&self, session: &str) -> Result<HashSet<String>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[hook-handle]]
