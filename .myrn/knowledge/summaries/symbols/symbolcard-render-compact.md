---
id: symbolcard-render-compact
type: summary
status: unverified
title: SymbolCard::render_compact
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard::render_compact
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard::render_compact@4d9b8a5acb19c39d
---
# SymbolCard::render_compact

Renders a symbol as a short text card with a header line, signature (only when code is None), first line of the summary clipped to 400 chars, up to 3 calls and 3 called-by links, a one-line tests summary, up to 2 known records, and commits only when with_commits is true (max 2). If code is Some, it appends that code in a fenced block tagged with the node's lang, trimmed of trailing whitespace; returns the String.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:219-261`
- **Signature:** `pub fn render_compact(&self, code: Option<&str>, with_commits: bool) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[episode-clip]], [[cards-summary-text]], [[cards-row]], [[cards-list]], [[cards-kind-of]], [[cards-tests-summary]], [[node-location]]
- **Called by:** [[engine-context-excluding]]
