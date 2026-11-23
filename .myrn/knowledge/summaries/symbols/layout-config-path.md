---
id: layout-config-path
type: summary
status: unverified
title: Layout::config_path
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::config_path
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::config_path@5bc9fe10398d6a0a
map_version: '2'
---
# Layout::config_path

Returns the path to the project config file inside the repo's `.myrn` directory.

## Behaviour

- -> <repo>/.myrn/project.toml
- pure path join; the file need not exist

## Terms

project config, toml file, project settings, config file location

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:56-58`
- **Signature:** `pub fn config_path(&self) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-open-located]], [[engine-doctor]]
