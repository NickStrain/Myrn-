---
id: api-check-api
type: summary
status: unverified
title: api::check
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::check
anchors:
- sym:crates/myrn-engine/src/api.rs#api::check@bee085ab8d174fb0
map_version: '2'
---
# api::check

Builds a Check from a name, a level and any string-like detail.

## Behaviour

- Converts name and detail into owned strings, with no validation
- Used only by doctor

## Terms

diagnostic builder, doctor line, check helper

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:131-137`
- **Signature:** `fn check(name: &str, level: Level, detail: impl Into<String>) -> Check`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-doctor]]
