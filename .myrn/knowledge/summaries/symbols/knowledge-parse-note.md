---
id: knowledge-parse-note
type: summary
status: unverified
title: knowledge::parse_note
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::parse_note
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::parse_note@651f235a1f97c3ab
map_version: '2'
---
# knowledge::parse_note

Parses note text from a repo-relative path into a Note, passing the file stem and first path segment as folder to the frontmatter parser.

## Behaviour

- `abc.md` at root -> no folder, stem `abc`
- `notes/sub/abc.md` -> folder `notes`, stem `abc`
- a name without the .md suffix keeps its full name as the stem

## Terms

frontmatter, note parsing, file name, stem, folder

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:473-478`
- **Signature:** `fn parse_note(rel: &str, text: &str) -> Note`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[frontmatter-parse]]
- **Called by:** [[engine-knowledge-at]]
