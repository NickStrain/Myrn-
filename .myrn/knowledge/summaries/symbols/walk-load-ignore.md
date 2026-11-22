---
id: walk-load-ignore
type: summary
status: unverified
title: walk::load_ignore
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::load_ignore
anchors:
- sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::load_ignore@0acba9edfa348c27
---
# walk::load_ignore

Builds a Gitignore matcher rooted at `root` from `ignore_file` if it exists; returns an empty matcher on a missing file or build error, never failing.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/walk.rs:23-29`
- **Signature:** `pub fn load_ignore(root: &Path, ignore_file: &Path) -> Gitignore`
- **In:** [[file-crates-myrn-engine-src-indexer-walk-rs]]
- **Called by:** [[engine-capture]], [[walk-walk]]
