---
id: adapter-rel
type: summary
status: unverified
title: adapter::rel
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::rel
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::rel@ed226e6467c01fc0
map_version: '2'
---
# adapter::rel

Returns a path relative to root with forward slashes, or the full path if it is not under root.

## Behaviour

- root /repo, p /repo/src/a.rs -> 'src/a.rs'
- p outside root -> full path unchanged, with slashes normalised
- Backslashes are replaced with forward slashes (Windows paths)

## Terms

repo relative, display path, strip prefix, slash normalize

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:88-93`
- **Signature:** `pub fn rel(root: &Path, p: &Path) -> String`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]], [[codex-install]], [[codex-uninstall]], [[codex-inspect]]
