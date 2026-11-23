---
id: engine-touched-symbols
type: summary
status: unverified
title: Engine::touched_symbols
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::touched_symbols
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::touched_symbols@29365eb765985808
map_version: '2'
---
# Engine::touched_symbols

Re-parses the given files and returns ids of symbols whose body hash changed, appeared or disappeared.

## Behaviour

- Compares the symbol snapshot of the files before and after a refresh
- Result is sorted

## Terms

edit diff, changed symbols, body hash, symbol diff

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1000-1026`
- **Signature:** `fn touched_symbols(&mut self, files: &[String]) -> Result<Vec<String>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-refresh]]
- **Called by:** [[engine-capture]]
