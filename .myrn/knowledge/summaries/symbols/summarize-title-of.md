---
id: summarize-title-of
type: summary
status: unverified
title: summarize::title_of
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::title_of
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::title_of@38298fdabf75f9cb
---
# summarize::title_of

Gives the note title: Project for the root module, a path with a trailing slash for directories, the file path for files, and the last two qualified-name segments joined by :: for symbols.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:818-830`
- **Signature:** `fn title_of(n: &Node) -> String`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[seed-segments]]
- **Called by:** [[engine-summarize]], [[engine-map-body]]
