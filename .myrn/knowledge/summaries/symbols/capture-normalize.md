---
id: capture-normalize
type: summary
status: unverified
title: capture::normalize
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::normalize
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::normalize@8ce05c0a1c233a23
---
# capture::normalize

Lexically resolves '.' and '..' components without touching the filesystem, so symlinks are not resolved and '..' simply removes the previous component.

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:162-175`
- **Signature:** `fn normalize(p: &Path) -> std::path::PathBuf`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Called by:** [[capture-relative]]
