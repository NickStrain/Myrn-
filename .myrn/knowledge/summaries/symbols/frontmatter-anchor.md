---
id: frontmatter-anchor
type: summary
status: unverified
title: frontmatter::Anchor
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Anchor
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Anchor@a6c8b3ebd6ddd7ff
map_version: '2'
---
# frontmatter::Anchor

A symbol the note depends on, paired with the body hash it had when the note was written.

## Behaviour

- auth::session::refresh@9c1e44 -> target auth::session::refresh, hash 9c1e44
- no usable @hash part -> hash None, and the target is the text with trailing @ removed
- the split uses the last @, so everything before the final @ stays in the target

## Terms

anchor, dependency, symbol, body hash, qualified name, hash

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:61-64`
- **Signature:** `pub struct Anchor`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
