---
id: engine-init
type: summary
status: unverified
title: Engine::init
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::init
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::init@f7df3c8148ad84a4
---
# Engine::init

Creates the repository config directory, writing project.toml only when the id is not provisional, plus the ignore file and knowledge .gitkeep placeholders, then opens the store and refreshes. It is safe to run again and returns the engine with the IndexReport.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:181-205`
- **Signature:** `pub fn init(cwd: &Path, data_home: Option<&Path>) -> Result<(Engine, IndexReport)>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[api-locate]], [[engine-open-located]], [[engine-refresh]], [[config-load]], [[config-initial-toml]], [[layout-knowledge-dir]], [[layout-create-dirs]], [[resolver-exists]]
- **Called by:** [[setup]], [[repo-notes]], [[repo-packing]], [[repo-58c6]], [[repo-summarize]], [[engine]], [[init-run]]
