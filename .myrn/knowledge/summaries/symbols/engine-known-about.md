---
id: engine-known-about
type: summary
status: unverified
title: Engine::known_about
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::known_about
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::known_about@4dd69ef5c3dbb1db
map_version: '2'
---
# Engine::known_about

Lists earned records about a symbol: facts and decisions linked by about edges, and root causes linked by located_in along with the bugs they caused. Superseded records are left out.

## Behaviour

- Only records with origin earned are kept, with no duplicate ids
- Verified records sort before the others

## Terms

learned facts, known issues, root cause, decisions, provenance, superseded

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:474-505`
- **Signature:** `fn known_about(&self, id: &str) -> Result<Vec<Known>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[render-safe-text]], [[render-node-provenance]], [[graph-neighbors]]
- **Called by:** [[engine-symbol-card]]
