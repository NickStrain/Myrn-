---
id: filecard-render
type: summary
status: unverified
title: FileCard::render
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::FileCard::render
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::FileCard::render@acd6ef98bbc10d32
map_version: '2'
---
# FileCard::render

Produces the text card for a file: path, language, defined symbols, imports, importers and up to 5 changed-in commits. Symbols, imports and importers are each limited by limit.

## Behaviour

- path None -> the file row is empty
- lang None -> the language row shows ?
- changed-in row omitted when commits is empty

## Terms

file report, module card, source file view, file overview, import list

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:272-284`
- **Signature:** `pub fn render(&self, limit: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[cards-row]], [[cards-list]]
