---
id: layout-new
type: summary
status: unverified
title: Layout::new
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::new
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::new@fa1792481333d4af
map_version: '2'
---
# Layout::new

Builds a Layout from the repo root, data home, project id, and optional worktree key by joining paths. It does no filesystem I/O and no validation.

## Behaviour

- worktree Some("wt1") -> db_path = <data_home>/projects/<id>/worktrees/wt1/memory.db
- worktree None -> db_path = <data_home>/projects/<id>/memory.db
- repo_dir = <repo_root>/.myrn; project_id and worktree key are not sanitised

## Terms

layout constructor, worktree database, memory db path, project id, linked worktree, path construction

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:37-54`
- **Signature:** `pub fn new( repo_root: &Path, data_home: &Path, project_id: &str, worktree: Option<&str>, ) -> Layout`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-open-located]]
- **Tests:** `worktrees_get_their_own_db`
