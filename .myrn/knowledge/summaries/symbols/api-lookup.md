---
id: api-lookup
type: summary
status: unverified
title: api::Lookup
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Lookup
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Lookup@4261cf12fd985f81
map_version: '2'
---
# api::Lookup

Outcome of resolving a name: one Found value, Ambiguous with candidate links, or NotFound with the query. Serialised with a result tag.

## Behaviour

- Ambiguous is produced by pick when several matches do not clearly single out one
- Serialised result values are found, ambiguous and not_found

## Terms

match result, name resolution, ambiguous match, candidates, not found

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:36-40`
- **Signature:** `pub enum Lookup<T>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
