---
id: note-display-title
type: summary
status: unverified
title: Note::display_title
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::display_title
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::display_title@d6516a8dc6e8e93a
map_version: '2'
---
# Note::display_title

Returns the header title if it is not blank, otherwise the first non-empty body line with leading hashes removed, cut to 80 characters, otherwise the id.

## Behaviour

- title ' Foo ' -> Foo
- no title and body '# Heading' followed by text -> Heading
- no title and empty body -> the id; a first line over 80 characters -> its first 80 characters plus an ellipsis

## Terms

title, heading, display name, label, short name, summary line

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:194-214`
- **Signature:** `pub fn display_title(&self) -> String`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[engine-records]], [[engine-record-row]], [[engine-diff-knowledge]], [[engine-review-items]], [[engine-viz-notes-graph]], [[sync-link]]
- **Tests:** `parses_the_design_example`
