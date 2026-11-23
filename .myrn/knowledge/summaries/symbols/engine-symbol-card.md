---
id: engine-symbol-card
type: summary
status: unverified
title: Engine::symbol_card
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::symbol_card
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::symbol_card@a213cc9882dbb41e
map_version: '2'
---
# Engine::symbol_card

Builds a symbol's card: known records, outdated-summary flag, callers and callees, members, direct tests (including those of its methods), and up to 3 recent commits.

## Behaviour

- Tests of a type include tests of its members, with duplicates removed by id
- called_by leaves out test nodes
- commits capped at 3

## Terms

symbol summary, card, callers, callees, test coverage, members

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:439-470`
- **Signature:** `fn symbol_card(&self, n: Node) -> Result<SymbolCard>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-links]], [[engine-commits-touching]], [[engine-known-about]], [[engine-summary-outdated]], [[link-new]], [[walks-members]], [[walks-direct-tests]], [[graph-neighbors]]
- **Called by:** [[engine-explain]], [[engine-context-excluding]]
