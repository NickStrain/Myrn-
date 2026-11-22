---
id: config-initial-toml
type: summary
status: unverified
title: Config::initial_toml
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::Config::initial_toml
anchors:
- sym:crates/myrn-engine/src/config.rs#config::Config::initial_toml@7bb8b25aa52d5652
---
# Config::initial_toml

Builds the commented TOML text for `myrn init` from the default values, inserting `project_id` into the `[project]` id string without escaping, so it must not contain quotes or backslashes.

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:140-188`
- **Signature:** `pub fn initial_toml(project_id: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
- **Called by:** [[engine-init]]
