---
id: filecard-render
type: summary
status: unverified
title: FileCard::render
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::FileCard::render
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::FileCard::render@acd6ef98bbc10d32
---
# FileCard::render

Renders a file card: a 'file' header with the path, the language (default '?'), then defines, imports, and imported by lists capped at limit, plus up to five changed-in commits.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:265-277`
- **Signature:** `pub fn render(&self, limit: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[cards-row]], [[cards-list]]
