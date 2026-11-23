---
id: sharing-local-paths
type: summary
status: unverified
title: sharing::local_paths
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::local_paths
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::local_paths@7fe9d642e58066ed
map_version: '2'
---
# sharing::local_paths

Rewrites each space-separated token: paths under the repository root become relative, the root itself becomes a dot, and other absolute paths become a placeholder.

## Behaviour

- 'pytest /repo/tests/x.py' with root /repo -> 'pytest tests/x.py'
- token equal to the root -> '.'
- 'cargo test --manifest-path /tmp/x/Cargo.toml' -> 'cargo test --manifest-path <path>'

## Terms

redact path, relative path, absolute path, hide path, local path, windows path, path scrub

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:257-291`
- **Signature:** `fn local_paths(s: &str, root: &Path) -> String`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
- **Called by:** [[sharing-clean]]
- **Tests:** `evidence_paths_stay_local`
