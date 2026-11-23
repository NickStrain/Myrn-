---
id: config-initial-toml
type: summary
status: unverified
title: Config::initial_toml
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::Config::initial_toml
anchors:
- sym:crates/myrn-engine/src/config.rs#config::Config::initial_toml@e391f8e1c23b534a
map_version: '2'
---
# Config::initial_toml

Builds the project.toml text that myrn init writes, using the given project id and the Default values for the other settings. It returns the string and writes nothing.

## Behaviour

- project_id 'abc' -> [project] id = 'abc'
- project_id is placed inside quotes without escaping
- no [embeddings] section is written, so those values stay at defaults

## Terms

init template, generated config, default toml, project file template, myrn init

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:198-252`
- **Signature:** `pub fn initial_toml(project_id: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
- **Called by:** [[engine-init]]
