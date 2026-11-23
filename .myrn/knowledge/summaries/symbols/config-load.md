---
id: config-load
type: summary
status: unverified
title: Config::load
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::Config::load
anchors:
- sym:crates/myrn-engine/src/config.rs#config::Config::load@0b6d00c90be09e09
map_version: '2'
---
# Config::load

Reads and parses the TOML file at path, returning the defaults without reading anything if the file does not exist.

## Behaviour

- path missing -> Ok(Config::default()), no read
- invalid TOML -> Err with context 'reading <path>'
- file exists but cannot be read -> Err from read_to_string with no path context

## Terms

load config, read project.toml, parse toml, missing file default, config path

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:189-195`
- **Signature:** `pub fn load(path: &Path) -> Result<Config>`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
- **Called by:** [[engine-init]], [[engine-open]], [[engine-open-located]], [[engine-doctor]]
