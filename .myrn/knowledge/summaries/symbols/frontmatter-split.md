---
id: frontmatter-split
type: summary
status: unverified
title: frontmatter::split
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::split
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::split@bff61b80a9b685c2
---
# frontmatter::split

Separates text into (header, body, problem) at the first closing '---' or '...' line; strips a leading BOM and accepts CRLF. Missing opening '---' gives 'no YAML header' with the whole text as body; missing closing gives 'header has no closing ---' with the rest as body.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:156-186`
- **Signature:** `fn split(text: &str) -> (Option<&str>, &str, Option<String>)`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[frontmatter-parse]]
