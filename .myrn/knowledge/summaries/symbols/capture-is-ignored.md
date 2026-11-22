---
id: capture-is-ignored
type: summary
status: unverified
title: capture::is_ignored
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::is_ignored
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::is_ignored@ebe8989b9cef594c
---
# capture::is_ignored

Returns true when the repo-relative path, or any parent directory, matches the Gitignore rules.

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:137-139`
- **Signature:** `fn is_ignored(ignore: &Gitignore, rel: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Called by:** [[capture-to-entry]], [[capture-hide-ignored]]
