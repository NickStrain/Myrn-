---
id: capture-normalize
type: summary
status: unverified
title: capture::normalize
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::normalize
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::normalize@8ce05c0a1c233a23
map_version: '2'
---
# capture::normalize

Lexically resolves `.` and `..` components without touching the file system. It does not check that the path exists.

## Behaviour

- "/r/src/./a.rs" -> "/r/src/a.rs"; "/r/src/../a.rs" -> "/r/a.rs"
- "/../x" -> "/x", so `..` above the root is clamped
- "a/../../b" -> "b", so a leading `..` on a relative path is silently dropped

## Terms

dot segments, path cleanup, lexical path, parent dir, no filesystem access, dot-dot removal

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:162-175`
- **Signature:** `fn normalize(p: &Path) -> std::path::PathBuf`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Called by:** [[capture-relative]]
