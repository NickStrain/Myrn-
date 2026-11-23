---
id: summarize-slug
type: summary
status: unverified
title: summarize::slug
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::slug
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::slug@d54b7fbec885f95f
map_version: '2'
---
# summarize::slug

Lowercases ASCII letters and digits, turns any other run of characters into one '-', strips leading and trailing '-', and returns 'x' when nothing remains.

## Behaviour

- 'crates/myrn/src/main.rs' -> 'crates-myrn-src-main-rs'
- '__init__' -> 'init'; '...' -> 'x'
- non-ASCII characters act as separators, not transliterated: 'café' -> 'caf'

## Terms

slugify, filename safe, note id, kebab case, ascii slug, path to id

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:933-949`
- **Signature:** `pub fn slug(s: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Called by:** [[engine-summarize]]
- **Tests:** `slugs_are_obsidian_safe`
