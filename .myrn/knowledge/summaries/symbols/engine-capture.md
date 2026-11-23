---
id: engine-capture
type: summary
status: unverified
title: Engine::capture
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::capture
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::capture@95ff6a6fa4a1a576
map_version: '2'
---
# Engine::capture

Writes one hook event to the session's episode log when the developer has consented and the agent is connected, then updates the episode node. Returns whether anything was written.

## Behaviour

- Returns false without writing when consent is absent or the event yields no entry
- A tool file edit re-parses the listed files and records symbols whose body changed, appeared or vanished
- A shell command that changed files adds a separate entry marked via shell; at session end, commits since the session started are attached

## Terms

hook event, session log, episode recording, consent, agent hook, capture event

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:894-953`
- **Signature:** `pub fn capture(&mut self, agent: &str, event: &myrn_model::event::Event) -> Result<bool>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-state]], [[engine-shell-changes]], [[engine-touched-symbols]], [[api-episode-key]], [[consent-may-record]], [[episode-fold]], [[episode-persist]], [[capture-to-entry]], [[outcome-commits-since]], [[walk-load-ignore]], [[episodes-path-for]], [[episodes-append]], [[episodes-read]], [[layout-ignore-path]], [[layout-episodes-dir]]
- **Called by:** [[hook-handle]]
- **Tests:** `shell_edits_are_captured_and_lead_to_a_record_request`
