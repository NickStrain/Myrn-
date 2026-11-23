---
id: engine-file-card
type: summary
status: unverified
title: Engine::file_card
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::file_card
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::file_card@7a797b6661fbbb72
map_version: '2'
---
# Engine::file_card

Builds a file's card: its symbols ordered by start line, what it imports, what imports it, and up to 5 recent commits.

## Behaviour

- Symbols are sorted by start line
- imports comes from outgoing imports edges and imported_by from incoming ones

## Terms

file summary, module imports, dependents, file commits, file card

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:507-519`
- **Signature:** `fn file_card(&self, n: Node) -> Result<FileCard>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-links]], [[engine-commits-touching]], [[link-new]], [[graph-neighbors]]
- **Called by:** [[engine-explain]]
