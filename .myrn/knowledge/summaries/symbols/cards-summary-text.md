---
id: cards-summary-text
type: summary
status: unverified
title: cards::summary_text
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::summary_text
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::summary_text@9fc28490702f93bb
map_version: '2'
---
# cards::summary_text

Chooses the text shown as a symbol's summary: the stored summary, prefixed when outdated, otherwise the doc comment, otherwise None. Returns a Cow, borrowed except when the outdated prefix is added.

## Behaviour

- summary Some and outdated true -> "(possibly outdated: the code changed since) " followed by the summary
- summary Some and outdated false -> summary returned unchanged
- summary None -> doc used even if outdated is true; None when both are missing

## Terms

doc comment, docstring, stale marker, fallback text, summary selection

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:69-75`
- **Signature:** `fn summary_text<'a>(n: &'a Node, outdated: bool) -> Option<std::borrow::Cow<'a, str>>`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Called by:** [[symbolcard-render]], [[symbolcard-render-compact]]
