---
id: engine-file-card
type: summary
status: unverified
title: Engine::file_card
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::file_card
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::file_card@7a797b6661fbbb72
---
# Engine::file_card

Builds a FileCard with the defined symbols sorted by start line, the imports and importers, and up to 5 commits.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:478-490`
- **Signature:** `fn file_card(&self, n: Node) -> Result<FileCard>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-links]], [[engine-commits-touching]], [[link-new]], [[graph-neighbors]]
- **Called by:** [[engine-explain]]
