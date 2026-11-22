---
id: config-load
type: summary
status: unverified
title: Config::load
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::Config::load
anchors:
- sym:crates/myrn-engine/src/config.rs#config::Config::load@0b6d00c90be09e09
---
# Config::load

Parses the TOML at `path` into `Config`, returning defaults without error if the file is missing and adding the path as context when parsing fails. Other read errors are returned without that context.

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:131-137`
- **Signature:** `pub fn load(path: &Path) -> Result<Config>`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
- **Called by:** [[engine-init]], [[engine-open]], [[engine-open-located]], [[engine-doctor]]
