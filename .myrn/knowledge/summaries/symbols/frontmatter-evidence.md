---
id: frontmatter-evidence
type: summary
status: unverified
title: frontmatter::Evidence
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Evidence
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Evidence@6e2b9d22e7e96af6
map_version: '2'
---
# frontmatter::Evidence

One evidence entry from the header, holding a kind and its value, or an empty kind for a plain string.

## Behaviour

- - test: tests/x.rs -> kind test, value tests/x.rs
- - plain words -> kind empty, value plain words
- a map with more than one key, or a non-scalar value -> not stored, and parse reports it as not kind: value

## Terms

evidence, proof, test, commit, kind, supporting reference

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:68-71`
- **Signature:** `pub struct Evidence`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
