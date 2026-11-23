---
id: knowledge-walk-notes
type: summary
status: unverified
title: knowledge::walk_notes
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::walk_notes
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::walk_notes@2e712f7ee8cc95e6
map_version: '2'
---
# knowledge::walk_notes

Recursively visits `.md` files under a directory, skipping dot-prefixed names, and calls the callback with each file's repo-relative path and text.

## Behaviour

- unreadable directory or file -> silently skipped
- file outside the repo root (no repo-relative path) -> skipped
- non-.md files are ignored

## Terms

scan, traverse, recurse, directory walk, markdown files, read notes

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:480-499`
- **Signature:** `fn walk_notes(root: &Path, dir: &Path, f: &mut dyn FnMut(&str, &str))`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[knowledge-walk-notes]], [[paths-repo-rel]]
- **Called by:** [[engine-knowledge-at]], [[knowledge-walk-notes]]
