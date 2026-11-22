---
id: engine-capture
type: summary
status: unverified
title: Engine::capture
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::capture
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::capture@95ff6a6fa4a1a576
---
# Engine::capture

Appends one hook event to the session episode log when the developer has consented for this agent, and returns false when nothing was written. Edit results re-parse the touched files to find changed symbols, shell commands refresh the index and record changed files as a separate shell edit, and session end records up to 50 commits since the session began; the log is then folded into the episode node.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:828-887`
- **Signature:** `pub fn capture(&mut self, agent: &str, event: &myrn_model::event::Event) -> Result<bool>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-state]], [[engine-shell-changes]], [[engine-touched-symbols]], [[api-episode-key]], [[consent-may-record]], [[episode-fold]], [[episode-persist]], [[capture-to-entry]], [[outcome-commits-since]], [[walk-load-ignore]], [[episodes-path-for]], [[episodes-append]], [[episodes-read]], [[layout-ignore-path]], [[layout-episodes-dir]]
- **Called by:** [[hook-handle]]
- **Tests:** `shell_edits_are_captured_and_lead_to_a_record_request`
