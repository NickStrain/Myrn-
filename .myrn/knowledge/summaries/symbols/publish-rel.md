---
id: publish-rel
type: summary
status: unverified
title: publish::rel
source: myrn summarize
about: sym:crates/myrn/src/cmd/publish.rs#cmd::publish::rel
anchors:
- sym:crates/myrn/src/cmd/publish.rs#cmd::publish::rel@5e025b7a62a27275
map_version: '2'
---
# publish::rel

Returns a path relative to the repo root for display, with forward slashes, or the unchanged path if it is outside the root.

## Behaviour

- root /a and path /a/.myrn/knowledge/x.md -> ".myrn/knowledge/x.md"
- path outside root -> full path returned unchanged, with separators normalized
- Windows-style separators are replaced with forward slashes

## Terms

relative path, display path, strip root, path separator, windows separator, normalize slashes, output path

## Connections

- **Location:** `crates/myrn/src/cmd/publish.rs:50-55`
- **Signature:** `fn rel(p: &Path, root: &Path) -> String`
- **In:** [[file-crates-myrn-src-cmd-publish-rs]]
- **Called by:** [[publish-run]], [[publish-show]]
