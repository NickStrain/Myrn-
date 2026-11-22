---
id: layout-new
type: summary
status: unverified
title: Layout::new
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::new
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::new@fa1792481333d4af
---
# Layout::new

Builds a Layout from the repo root, data home, project id, and optional worktree key. data_dir is data_home/projects/<id>, and db_path is memory.db there, or under worktrees/<key>/ for linked worktrees. This is pure path computation with no filesystem access.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:37-54`
- **Signature:** `pub fn new( repo_root: &Path, data_home: &Path, project_id: &str, worktree: Option<&str>, ) -> Layout`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-open-located]]
- **Tests:** `worktrees_get_their_own_db`
