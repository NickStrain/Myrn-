---
id: layout-create-dirs
type: summary
status: unverified
title: Layout::create_dirs
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::create_dirs
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::create_dirs@95e5521e9244d280
---
# Layout::create_dirs

Creates knowledge/<kind> and notes/<kind> for each of the five KNOWLEDGE_KINDS, plus the episodes and evidence directories and the parent of db_path, using create_dir_all. Unlike the other methods, this writes to disk and returns an error on any IO failure.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:80-91`
- **Signature:** `pub fn create_dirs(&self) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Calls:** [[layout-knowledge-dir]], [[layout-notes-dir]], [[layout-episodes-dir]], [[layout-evidence-dir]]
- **Called by:** [[engine-init]]
