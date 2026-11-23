---
id: config-config
type: summary
status: unverified
title: config::Config
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::Config
anchors:
- sym:crates/myrn-engine/src/config.rs#config::Config@d049313668f29be5
map_version: '2'
---
# config::Config

Root settings type with one field per project.toml section. Each section falls back to its Default when absent.

## Behaviour

- empty file -> every section takes its default
- missing [ranking] table -> RankingConfig::default values
- unknown keys are not rejected here (no deny_unknown_fields)

## Terms

project settings, configuration root, toml sections, project.toml, defaults

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:9-24`
- **Signature:** `pub struct Config`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
