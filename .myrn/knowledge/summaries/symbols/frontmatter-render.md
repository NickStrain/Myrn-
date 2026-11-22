---
id: frontmatter-render
type: summary
status: unverified
title: frontmatter::render
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::render
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::render@4ba1744f1fca0ea9
---
# frontmatter::render

Serializes a Note to '---' YAML header plus body with a fixed key order (id, type, status, title, confidence, source, signature, grouped relation keys, anchors, evidence, extra), writing link targets for which note_ids returns true as '[[target]]' and ensuring a trailing newline. Multiple links with the same relation are merged into one sequence, so output order groups by relation rather than preserving input order.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:387-468`
- **Signature:** `pub fn render(note: &Note, note_ids: &dyn Fn(&str) -> bool) -> String`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[engine-put-note]], [[engine-summarize]], [[notes-write]]
- **Tests:** `round_trips`, `round_trips_lists_extra_and_crlf`
