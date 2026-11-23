---
id: api-engine
type: summary
status: unverified
title: api::Engine
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine@457fcea86f481533
map_version: '2'
---
# api::Engine

Holds the project layout, config, project id, the open database and the time of the last refresh. Every engine operation is a method on it.

## Behaviour

- Built only by open_located, which init and open both call
- provisional_id is true when the repository has no commits yet and the id is a stand-in hash

## Terms

project handle, store handle, database handle, repository context, engine state

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:23-31`
- **Signature:** `pub struct Engine`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
