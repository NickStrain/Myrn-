---
id: capture-relative
type: summary
status: unverified
title: capture::relative
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::relative
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::relative@cddde93c6519bf18
---
# capture::relative

Returns the repo-relative forward-slash path for a reported path, resolving relative paths against cwd and normalizing '.' and '..' lexically. Returns None for paths outside root, for the repo root itself, and for anything under .git; if the direct comparison fails, it retries with canonicalized root and path (for example through symlinks).

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:142-159`
- **Signature:** `pub fn relative(root: &Path, cwd: &Path, path: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Calls:** [[capture-normalize]], [[paths-repo-rel]]
- **Called by:** [[capture-to-entry]], [[capture-hide-ignored]]
