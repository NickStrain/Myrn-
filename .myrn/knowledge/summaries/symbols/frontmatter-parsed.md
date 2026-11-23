---
id: frontmatter-parsed
type: summary
status: unverified
title: frontmatter::Parsed
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Parsed
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Parsed@1c62c754ce4c26bd
map_version: '2'
---
# frontmatter::Parsed

A parsed note together with the list of problems found while reading it.

## Behaviour

- an empty problems list means the header was clean
- parsing never fails, so a note with problems is still returned and indexed
- problems are plain sentences describing each field that was wrong

## Terms

problems, diagnostics, warnings, parse result, malformed header, validation

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:75-78`
- **Signature:** `pub struct Parsed`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
