---
id: engine-known-about
type: summary
status: unverified
title: Engine::known_about
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::known_about
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::known_about@033debb07176f5b9
---
# Engine::known_about

Returns earned records about an id: those linked by about edges, plus root causes linked by located_in and the bugs they caused_by. Superseded records are dropped and verified ones sort first.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:449-476`
- **Signature:** `fn known_about(&self, id: &str) -> Result<Vec<Known>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[graph-neighbors]]
- **Called by:** [[engine-symbol-card]]
