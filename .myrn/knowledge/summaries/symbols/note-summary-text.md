---
id: note-summary-text
type: summary
status: unverified
title: Note::summary_text
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::summary_text
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::summary_text@bd7cc06fe6c4343d
---
# Note::summary_text

Returns the body with leading blank lines and a leading '# ' heading removed, cut at the first '## ' line and trimmed; used as the text of summary notes.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:114-129`
- **Signature:** `pub fn summary_text(&self) -> String`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[engine-summarize]], [[sync-link]]
