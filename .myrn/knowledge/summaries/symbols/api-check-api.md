---
id: api-check-api
type: summary
status: unverified
title: api::check
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::check
anchors:
- sym:crates/myrn-engine/src/api.rs#api::check@bee085ab8d174fb0
---
# api::check

Builds a Check from a name, a Level and a detail that converts to String.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:129-135`
- **Signature:** `fn check(name: &str, level: Level, detail: impl Into<String>) -> Check`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-doctor]]
