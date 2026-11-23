---
id: explained-render
type: summary
status: unverified
title: Explained::render
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Explained::render
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Explained::render@d6026f562811d431
map_version: '2'
---
# Explained::render

Renders the card as text, passing a line limit of 12 for a symbol and 25 for a file.

## Behaviour

- Symbol -> SymbolCard render with limit 12
- File -> FileCard render with limit 25

## Terms

card text, summary output, line limit, output size

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:68-73`
- **Signature:** `pub fn render(&self) -> String`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
