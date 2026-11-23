---
id: anchors-anchored-path
type: summary
status: unverified
title: anchors::anchored_path
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/anchors.rs#validity::anchors::anchored_path
anchors:
- sym:crates/myrn-engine/src/validity/anchors.rs#validity::anchors::anchored_path@fb933046370568a1
map_version: '2'
---
# anchors::anchored_path

Strips the 'sym:', 'test:' or 'file:' prefix from an anchor id and returns the repo-relative path before any '#', or None for other prefixes.

## Behaviour

- "sym:src/a.rs#auth::refresh" -> Some("src/a.rs")
- "file:src/a.rs" -> Some("src/a.rs")
- "commit:abc" -> None (unknown prefix)

## Terms

anchor prefix, repo-relative path, symbol id parsing, file anchor, fragment stripping, sym prefix, test anchor

## Connections

- **Location:** `crates/myrn-engine/src/validity/anchors.rs:88-94`
- **Signature:** `fn anchored_path(id: &str) -> Option<&str>`
- **In:** [[file-crates-myrn-engine-src-validity-anchors-rs]]
- **Called by:** [[engine-suspect-notice]]
- **Tests:** `anchored_paths`
