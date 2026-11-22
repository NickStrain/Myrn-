---
id: engine-open
type: summary
status: unverified
title: Engine::open
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::open
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::open@cb2af3e7421f94fc
---
# Engine::open

Opens an initialised project without refreshing the index. It fails with a message to run myrn init when neither project.toml nor the ignore file exists.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:208-221`
- **Signature:** `pub fn open(cwd: &Path, data_home: Option<&Path>) -> Result<Engine>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[api-locate]], [[engine-open-located]], [[config-load]], [[resolver-exists]]
- **Called by:** [[runner-collect-injections]], [[connect-run]], [[disconnect-run]], [[doctor-run]], [[hook-handle]], [[ctx-engine]], [[rebuild-run]]
