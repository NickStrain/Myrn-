---
id: engine-symbol-card
type: summary
status: unverified
title: Engine::symbol_card
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::symbol_card
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::symbol_card@a213cc9882dbb41e
---
# Engine::symbol_card

Builds a SymbolCard: known records, the outdated-summary flag, calls, callers with test nodes excluded, members, deduplicated tests including those of its members, and up to 3 commits.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:414-445`
- **Signature:** `fn symbol_card(&self, n: Node) -> Result<SymbolCard>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-links]], [[engine-commits-touching]], [[engine-known-about]], [[engine-summary-outdated]], [[link-new]], [[walks-members]], [[walks-direct-tests]], [[graph-neighbors]]
- **Called by:** [[engine-explain]], [[engine-context-excluding]]
