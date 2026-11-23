---
id: config-projectconfig
type: summary
status: unverified
title: config::ProjectConfig
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::ProjectConfig
anchors:
- sym:crates/myrn-engine/src/config.rs#config::ProjectConfig@eccc71d6f3fb0ec3
map_version: '2'
---
# config::ProjectConfig

Holds the [project] table. Its id field caches the repository's root commit.

## Behaviour

- id absent or [project] missing -> id None
- id set in TOML -> Some with that string
- myrn init writes the project id it is given into this field

## Terms

project id, root commit, repository identity, cached root commit, project identifier

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:137-140`
- **Signature:** `pub struct ProjectConfig`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
