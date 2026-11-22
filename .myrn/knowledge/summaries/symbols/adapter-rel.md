---
id: adapter-rel
type: summary
status: unverified
title: adapter::rel
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::rel
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::rel@ed226e6467c01fc0
---
# adapter::rel

Returns p relative to root with forward slashes; if p is not under root, returns p unchanged apart from the slash conversion.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:88-93`
- **Signature:** `pub fn rel(root: &Path, p: &Path) -> String`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]], [[codex-install]], [[codex-uninstall]], [[codex-inspect]]
