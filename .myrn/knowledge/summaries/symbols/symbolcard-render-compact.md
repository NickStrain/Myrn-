---
id: symbolcard-render-compact
type: summary
status: unverified
title: SymbolCard::render_compact
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard::render_compact
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard::render_compact@6e5b5983439f06eb
map_version: '2'
---
# SymbolCard::render_compact

Produces the short card injected into prompts: header, a one-line summary clipped to 400 characters, up to 3 calls and 3 callers, optional tests and commits lines, up to 2 learned records, and an optional fenced code block. The signature appears only when no code is passed.

## Behaviour

- code Some -> signature omitted and code appended in a fence labelled with the node lang, trailing whitespace trimmed
- summary uses only its first line, clipped to 400 characters
- with_tests false -> no tests line; with_commits false -> no changed-in line

## Terms

prompt card, compact view, injected context, code excerpt, short summary

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:224-268`
- **Signature:** `pub fn render_compact(&self, code: Option<&str>, with_commits: bool, with_tests: bool) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[episode-clip]], [[cards-summary-text]], [[cards-row]], [[cards-list]], [[cards-kind-of]], [[cards-tests-summary]], [[node-location]]
- **Called by:** [[engine-context-excluding]]
