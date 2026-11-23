---
id: engine-init
type: summary
status: unverified
title: Engine::init
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::init
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::init@f7df3c8148ad84a4
map_version: '2'
---
# Engine::init

Creates the project config and ignore files when absent, creates the knowledge folders and local store, then refreshes the index and returns the report.

## Behaviour

- Writes project.toml only when it is absent and the project id is not provisional
- Writes the ignore file from the initial template only when it is absent, so running it again is safe
- Returns the engine together with the index report

## Terms

setup, bootstrap, first run, initialise, create store, index report

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:183-207`
- **Signature:** `pub fn init(cwd: &Path, data_home: Option<&Path>) -> Result<(Engine, IndexReport)>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[api-locate]], [[engine-open-located]], [[engine-refresh]], [[config-load]], [[config-initial-toml]], [[layout-knowledge-dir]], [[layout-create-dirs]], [[resolver-exists]]
- **Called by:** [[setup]], [[setup-history]], [[repo-learning]], [[repo-notes]], [[repo-packing]], [[repo-58c6]], [[two-clones]], [[repo-summarize]], [[repo-48c3]], [[repo-viz]], [[engine]], [[init-run]]
- **Tests:** `worktrees_share_notes_but_judge_them_by_their_own_files`, `shared_records_from_a_teammate_apply_when_their_code_matches`
