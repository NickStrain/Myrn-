---
id: engine-doctor
type: summary
status: unverified
title: Engine::doctor
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::doctor
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::doctor@14ec0d9079430ece
map_version: '2'
---
# Engine::doctor

Runs diagnostics on the repository, project id, data directory, database, grammars, index refresh, parse errors and notes, and returns the check results.

## Behaviour

- Project id is a warning when there are no commits yet, or when project.toml disagrees with the root commit
- The index refresh runs as part of the check; if it fails, the index check is a failure
- The agents check is always a warning saying none are connected

## Terms

health check, diagnostics, self test, troubleshoot, installation check

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1153-1297`
- **Signature:** `pub fn doctor(&mut self) -> Vec<Check>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[api-check-api]], [[engine-root]], [[engine-refresh]], [[engine-note-issues]], [[config-load]], [[git-root-commit]], [[lang-grammar]], [[db-schema-version]], [[db-integrity-ok]], [[layout-config-path]], [[sync-summary]]
- **Called by:** [[doctor-run]]
- **Tests:** `malformed_notes_and_dangling_links_are_flagged_not_dropped`
