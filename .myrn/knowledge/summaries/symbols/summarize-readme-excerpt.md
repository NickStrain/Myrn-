---
id: summarize-readme-excerpt
type: summary
status: unverified
title: summarize::readme_excerpt
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::readme_excerpt
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::readme_excerpt@77bcea8fe365134a
---
# summarize::readme_excerpt

Returns up to the first 4000 bytes (cut on a char boundary) of the first README variant found in the repo root, or None if there is none.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:799-816`
- **Signature:** `fn readme_excerpt(root: &Path) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Called by:** [[engine-summarize]]
