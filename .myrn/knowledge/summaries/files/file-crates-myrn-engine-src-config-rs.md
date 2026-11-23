---
id: file-crates-myrn-engine-src-config-rs
type: summary
status: unverified
title: crates/myrn-engine/src/config.rs
source: myrn summarize
about: file:crates/myrn-engine/src/config.rs
anchors:
- file:crates/myrn-engine/src/config.rs@14bc7ca4f2236f81
---
# crates/myrn-engine/src/config.rs

Defines the settings read from .myrn/project.toml as one serde struct per TOML section, each with its defaults, plus the loader that reads the file and the template text that myrn init writes for it. It also holds INITIAL_IGNORE, the .myrn/ignore template, which is not a listed symbol.

## Connections

- **Location:** `crates/myrn-engine/src/config.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[config-config]], [[config-rankingconfig]], [[rankingconfig-default]], [[config-embeddingsconfig]], [[embeddingsconfig-default]], [[config-summarizeconfig]], [[summarizeconfig-default]], [[config-recordsconfig]], [[recordsconfig-default]], [[config-projectconfig]], [[config-indexconfig]], [[indexconfig-default]], [[config-contextconfig]], [[contextconfig-default]], [[config-load]], [[config-initial-toml]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-learn-rs]], [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]]
