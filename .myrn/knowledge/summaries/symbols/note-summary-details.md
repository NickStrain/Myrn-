---
id: note-summary-details
type: summary
status: unverified
title: Note::summary_details
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::summary_details
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::summary_details@77253c81442a25bb
map_version: '2'
---
# Note::summary_details

Reads the Behaviour and Terms sections of a code-map note, returning bullet items and comma-separated terms.

## Behaviour

- '- a' and '- b' under the Behaviour heading -> bullets a and b
- a Terms line 'x, y , z' -> terms x, y, z; several Terms lines accumulate
- 'Behavior' spelling is also accepted; any other '## ' heading ends the section, and lines outside these sections are ignored

## Terms

behaviour, behavior, terms, bullets, code map, section, comma-separated, keywords

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:157-191`
- **Signature:** `pub fn summary_details(&self) -> (Vec<String>, Vec<String>)`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[engine-summarize]], [[sync-link]]
