---
id: note-display-title
type: summary
status: unverified
title: Note::display_title
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::display_title
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::display_title@d6516a8dc6e8e93a
---
# Note::display_title

Returns the trimmed header title if non-blank, else the first non-blank body line with leading '#' stripped, cut to 80 chars with an ellipsis; falls back to id when that line is empty.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:132-152`
- **Signature:** `pub fn display_title(&self) -> String`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[engine-review-items]], [[sync-link]]
- **Tests:** `parses_the_design_example`
