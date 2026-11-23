---
id: wikilink-normalize
type: summary
status: unverified
title: wikilink::normalize
source: myrn summarize
about: sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::normalize
anchors:
- sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::normalize@0cae3e1cd86326fc
map_version: '2'
---
# wikilink::normalize

Reduces a raw link string to a bare note name by dropping any |alias, #heading, directory path and trailing .md, then trimming. Returns None when nothing remains.

## Behaviour

- 'folder/Target.md#Heading|alias' -> Some('Target'); 'bugs/bug-7f3a91.md|the bug' -> Some('bug-7f3a91')
- 'diagram#Top' -> Some('diagram'); 'x.md.md' -> Some('x'); 'x.MD' -> Some('x.MD') because the .md strip is case-sensitive
- '', ' ', 'a/', '#Heading' or '.md' -> None

## Terms

link target, alias, heading anchor, basename, path prefix, md suffix, note name, file name

## Connections

- **Location:** `crates/myrn-store/src/notes/wikilink.rs:49-55`
- **Signature:** `pub fn normalize(raw: &str) -> Option<String>`
- **In:** [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Called by:** [[wikilink-extract]], [[wikilink-unwrap-link]]
