---
id: paths-repo-rel
type: summary
status: unverified
title: paths::repo_rel
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::repo_rel
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::repo_rel@e12db4553c15015c
map_version: '2'
---
# paths::repo_rel

Returns the path relative to the root as a string with forward slashes, or None if the path is not under the root.

## Behaviour

- root /r, path /r/a/b.rs -> Some("a/b.rs")
- path not under root -> None
- path equal to root -> Some("")

## Terms

relative path, repo-relative path, forward slash path, normalized path, note id path, portable path

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:95-102`
- **Signature:** `pub fn repo_rel(root: &Path, path: &Path) -> Option<String>`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[capture-relative]], [[walk-walk]], [[knowledge-walk-notes]], [[hook-handle]]
- **Tests:** `rel_paths_use_forward_slashes`
