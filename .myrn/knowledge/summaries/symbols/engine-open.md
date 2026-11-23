---
id: engine-open
type: summary
status: unverified
title: Engine::open
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::open
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::open@cb2af3e7421f94fc
map_version: '2'
---
# Engine::open

Opens an already set-up project without refreshing the index.

## Behaviour

- Errors telling the user to run myrn init when neither project.toml nor the ignore file exists
- Reads project.toml before resolving the project id

## Terms

load project, open store, existing project, no refresh

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:210-223`
- **Signature:** `pub fn open(cwd: &Path, data_home: Option<&Path>) -> Result<Engine>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[api-locate]], [[engine-open-located]], [[config-load]], [[resolver-exists]]
- **Called by:** [[runner-collect-injections]], [[main-simcheck]], [[connect-run]], [[disconnect-run]], [[doctor-run]], [[hook-handle]], [[ctx-engine]], [[rebuild-run]]
