---
id: paths-layout
type: summary
status: unverified
title: paths::Layout
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout@c3d5f1aa0a352894
map_version: '2'
---
# paths::Layout

Holds the resolved paths for one project: repo root, `.myrn` dir, project data dir, and the database path.

## Behaviour

- fields are filled only by Layout::new; no filesystem access happens in the struct
- db_path differs per linked worktree, so each worktree has its own database
- all fields are public PathBufs

## Terms

project layout, path set, project paths, path struct, resolved locations

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:26-34`
- **Signature:** `pub struct Layout`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
