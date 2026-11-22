---
id: engine-touched-symbols
type: summary
status: unverified
title: Engine::touched_symbols
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::touched_symbols
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::touched_symbols@29365eb765985808
---
# Engine::touched_symbols

Refreshes the index and returns the ids of symbols in the given files whose body hash changed, was added or was removed.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:934-960`
- **Signature:** `fn touched_symbols(&mut self, files: &[String]) -> Result<Vec<String>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-refresh]]
- **Called by:** [[engine-capture]]
