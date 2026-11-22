---
id: cards-summary-text
type: summary
status: unverified
title: cards::summary_text
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::summary_text
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::summary_text@9fc28490702f93bb
---
# cards::summary_text

Returns the summary to display. A stored summary gets a '(possibly outdated: the code changed since)' prefix when outdated is true; if no summary exists it falls back to the node's doc without the prefix, and returns None when neither is set.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:64-70`
- **Signature:** `fn summary_text<'a>(n: &'a Node, outdated: bool) -> Option<std::borrow::Cow<'a, str>>`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Called by:** [[symbolcard-render]], [[symbolcard-render-compact]]
