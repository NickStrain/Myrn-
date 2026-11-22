---
id: file-crates-myrn-store-src-notes-frontmatter-rs
type: summary
status: unverified
title: crates/myrn-store/src/notes/frontmatter.rs
source: myrn summarize
about: file:crates/myrn-store/src/notes/frontmatter.rs
anchors:
- file:crates/myrn-store/src/notes/frontmatter.rs@224f9f22ec665bdc
---
# crates/myrn-store/src/notes/frontmatter.rs

Parses and writes the YAML header of knowledge note Markdown files (id, type, status, typed links, anchors, evidence, extra fields) and defines the Note, Link, Anchor, Evidence and Parsed types they use. parse never fails: malformed input yields a Note with defaults plus a list of problems, and render writes a Note back as Markdown with a fixed header field order.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[frontmatter-note]], [[frontmatter-link]], [[frontmatter-anchor]], [[frontmatter-evidence]], [[frontmatter-parsed]], [[note-new]], [[note-summary-text]], [[note-display-title]], [[frontmatter-split]], [[frontmatter-scalar]], [[frontmatter-strings]], [[frontmatter-parse]], [[frontmatter-render]]
- **Imports:** [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-notes-layout-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-sync-rs]]
