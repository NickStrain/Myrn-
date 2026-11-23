---
id: frontmatter-render
type: summary
status: unverified
title: frontmatter::render
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::render
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::render@4ba1744f1fca0ea9
map_version: '2'
---
# frontmatter::render

Writes a note as Markdown with a YAML header in a fixed field order, writing link targets that note_ids accepts as wikilinks.

## Behaviour

- target accepted by note_ids -> written as [[target]]; other targets written plain
- one link under a key -> scalar value; several under the same key -> YAML list in first-seen order
- body not ending in a newline gets one appended; extra fields are written after evidence

## Terms

render, write, serialize, save, emit, markdown output, YAML header, wikilink

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:449-530`
- **Signature:** `pub fn render(note: &Note, note_ids: &dyn Fn(&str) -> bool) -> String`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[engine-put-note]], [[engine-publish-plan]], [[engine-summarize]], [[notes-write]]
- **Tests:** `round_trips`, `round_trips_lists_extra_and_crlf`
