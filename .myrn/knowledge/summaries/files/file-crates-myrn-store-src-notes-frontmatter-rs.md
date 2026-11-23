---
id: file-crates-myrn-store-src-notes-frontmatter-rs
type: summary
status: unverified
title: crates/myrn-store/src/notes/frontmatter.rs
source: myrn summarize
about: file:crates/myrn-store/src/notes/frontmatter.rs
anchors:
- file:crates/myrn-store/src/notes/frontmatter.rs@109e9ade982501f9
---
# crates/myrn-store/src/notes/frontmatter.rs

Typed YAML header parser and writer for note files. It defines the Note, Link, Anchor, Evidence and Parsed types, splits a file into header and body, records header problems instead of dropping fields, and renders a note back to Markdown with a deterministic header.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[frontmatter-note]], [[frontmatter-link]], [[frontmatter-anchor]], [[frontmatter-evidence]], [[frontmatter-parsed]], [[note-extra-str]], [[note-set-extra]], [[note-new]], [[note-summary-text]], [[note-summary-details]], [[note-display-title]], [[frontmatter-split]], [[frontmatter-scalar]], [[frontmatter-strings]], [[frontmatter-parse]], [[frontmatter-render]]
- **Imports:** [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-notes-layout-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-engine-src-validity-gc-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-sync-rs]]
