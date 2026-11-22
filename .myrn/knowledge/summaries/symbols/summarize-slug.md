---
id: summarize-slug
type: summary
status: unverified
title: summarize::slug
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::slug
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::slug@d54b7fbec885f95f
---
# summarize::slug

Lowercases ASCII alphanumerics and turns each run of other characters into one hyphen, trimming trailing hyphens and returning x for empty output.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:846-862`
- **Signature:** `pub fn slug(s: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Called by:** [[engine-summarize]]
- **Tests:** `slugs_are_obsidian_safe`
