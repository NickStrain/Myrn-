---
id: cards-filecard
type: summary
status: unverified
title: cards::FileCard
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::FileCard
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::FileCard@69cbac14681328e6
map_version: '2'
---
# cards::FileCard

Holds a file node with its defined symbols, imports, importers and changed-in commits. Render turns these into the file card text.

## Behaviour

- path None -> the file row is empty
- lang None -> the language row shows ?
- changed-in list is shown only when commits is non-empty

## Terms

file record, module entry, import graph, source file, defined symbols

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:78-84`
- **Signature:** `pub struct FileCard`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
