---
id: frontmatter-split
type: summary
status: unverified
title: frontmatter::split
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::split
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::split@bff61b80a9b685c2
map_version: '2'
---
# frontmatter::split

Splits raw text into header text, body text and an optional problem, dropping a leading byte-order mark first.

## Behaviour

- a first line of '---', header lines, then a closing '---' or '...' line -> header text, with the body after the closing line
- first line is not '---' -> no header, the whole text is the body, problem 'no YAML header'
- no closing line -> no header, and the rest of the file including the header lines becomes the body, problem 'header has no closing ---'

## Terms

front matter, header block, delimiter, byte order mark, BOM, closing fence, body, CRLF

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:218-248`
- **Signature:** `fn split(text: &str) -> (Option<&str>, &str, Option<String>)`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[frontmatter-parse]]
