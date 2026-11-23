---
id: note-summary-text
type: summary
status: unverified
title: Note::summary_text
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::summary_text
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::summary_text@bd7cc06fe6c4343d
map_version: '2'
---
# Note::summary_text

Returns the prose of a summary note: leading blank lines and one leading '# ' heading are dropped, and the text stops at the first line starting with '## '.

## Behaviour

- '# Title', blank line, 'Text', then '## Behaviour' -> 'Text'
- no leading heading -> every line up to the first '## ' line is kept
- an indented '## ' line does not end the text, because the match is on the raw line start

## Terms

summary, code summary, description, prose, intro, map text, overview

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:137-152`
- **Signature:** `pub fn summary_text(&self) -> String`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[engine-summarize]], [[sync-link]]
