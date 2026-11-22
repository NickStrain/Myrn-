---
id: file-crates-myrn-engine-src-config-rs
type: summary
status: unverified
title: crates/myrn-engine/src/config.rs
source: myrn summarize
about: file:crates/myrn-engine/src/config.rs
anchors:
- file:crates/myrn-engine/src/config.rs@793efb446706f5fd
---
# crates/myrn-engine/src/config.rs

Defines the schema for `.myrn/project.toml`: the `Config` struct and its section structs (project, index, context, records, summarize), each with defaults, plus `Config::load` and `Config::initial_toml`, which writes the file `myrn init` creates. Also holds `INITIAL_IGNORE`, the template for `.myrn/ignore`.

## Connections

- **Location:** `crates/myrn-engine/src/config.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[config-config]], [[config-summarizeconfig]], [[summarizeconfig-default]], [[config-recordsconfig]], [[recordsconfig-default]], [[config-projectconfig]], [[config-indexconfig]], [[indexconfig-default]], [[config-contextconfig]], [[contextconfig-default]], [[config-load]], [[config-initial-toml]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-lib-rs]]
