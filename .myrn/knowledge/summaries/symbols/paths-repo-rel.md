---
id: paths-repo-rel
type: summary
status: unverified
title: paths::repo_rel
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::repo_rel
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::repo_rel@e12db4553c15015c
---
# paths::repo_rel

Strips root from path and joins the remaining components with forward slashes, returning None if path is not under root. It does not canonicalize or touch the filesystem.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:95-102`
- **Signature:** `pub fn repo_rel(root: &Path, path: &Path) -> Option<String>`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[capture-relative]], [[walk-walk]]
- **Tests:** `rel_paths_use_forward_slashes`
