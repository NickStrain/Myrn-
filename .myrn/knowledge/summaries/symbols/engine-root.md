---
id: engine-root
type: summary
status: unverified
title: Engine::root
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::root
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::root@d563d4930e1c20fd
---
# Engine::root

Returns the repository root path.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:245-247`
- **Signature:** `pub fn root(&self) -> &Path`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-source-if-short]], [[engine-doctor]], [[engine-excerpt]], [[connect-run]], [[disconnect-run]], [[doctor-agent-checks]], [[init-run]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
