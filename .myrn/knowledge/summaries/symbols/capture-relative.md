---
id: capture-relative
type: summary
status: unverified
title: capture::relative
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::relative
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::relative@cddde93c6519bf18
map_version: '2'
---
# capture::relative

Turns a path into a repo-relative, forward-slash string, joining relative paths onto cwd and falling back to canonicalized paths for symlinks. Returns None for paths outside the repo, the repo root itself, and anything under .git.

## Behaviour

- root /r, cwd /r, path "/r/src/a.rs" -> "src/a.rs"
- cwd /r/src, path "../lib.rs" -> "lib.rs"
- "/etc/passwd", ".git/config" or the repo root itself -> None

## Terms

repo-relative path, path resolution, outside repository, git directory, symlink canonicalize, working directory join, dot-dot path

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:142-159`
- **Signature:** `pub fn relative(root: &Path, cwd: &Path, path: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Calls:** [[capture-normalize]], [[paths-repo-rel]]
- **Called by:** [[capture-to-entry]], [[capture-hide-ignored]]
