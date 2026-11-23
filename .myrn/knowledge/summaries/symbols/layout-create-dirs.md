---
id: layout-create-dirs
type: summary
status: unverified
title: Layout::create_dirs
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::create_dirs
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::create_dirs@95e5521e9244d280
map_version: '2'
---
# Layout::create_dirs

Creates the knowledge, notes, episodes, and evidence directory trees plus the database's parent directory. It returns the first I/O error.

## Behaviour

- uses create_dir_all, so existing directories are not an error
- creates knowledge/<kind> and notes/<kind> for each of the 5 KNOWLEDGE_KINDS
- an error part-way through returns early with no rollback, leaving earlier directories in place

## Terms

create directories, mkdir, setup dirs, init folders, directory tree, scaffold

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:80-91`
- **Signature:** `pub fn create_dirs(&self) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Calls:** [[layout-knowledge-dir]], [[layout-notes-dir]], [[layout-episodes-dir]], [[layout-evidence-dir]]
- **Called by:** [[engine-init]]
