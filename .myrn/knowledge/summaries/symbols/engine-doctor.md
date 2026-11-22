---
id: engine-doctor
type: summary
status: unverified
title: Engine::doctor
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::doctor
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::doctor@14ec0d9079430ece
---
# Engine::doctor

Runs the health checks (repository, project id against the root commit, data directory writability, database schema and integrity, tree-sitter grammars, a full refresh, parse errors, notes, agents) and returns one Check each. It refreshes the index as a side effect, and the agents check is always Warn with a fixed message.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1086-1230`
- **Signature:** `pub fn doctor(&mut self) -> Vec<Check>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[api-check-api]], [[engine-root]], [[engine-refresh]], [[engine-note-issues]], [[config-load]], [[git-root-commit]], [[lang-grammar]], [[db-schema-version]], [[db-integrity-ok]], [[layout-config-path]], [[sync-summary]]
- **Called by:** [[doctor-run]]
- **Tests:** `malformed_notes_and_dangling_links_are_flagged_not_dropped`
