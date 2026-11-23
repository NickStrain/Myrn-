---
id: render-pack
type: summary
status: unverified
title: render::pack
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/render.rs#pack::render::pack
anchors:
- sym:crates/myrn-engine/src/pack/render.rs#pack::render::pack@994605e4c5c4d72f
map_version: '2'
---
# render::pack

Assembles the pack text in order: header, cards for top items, an 'Also relevant' list of one-line entries, then quoted learned records, adding each chunk only if the budget allows. Returns the text together with the ids that were included.

## Behaviour

- no cards, rest or known items, or the budget cannot hold the header -> ('', [])
- a card, list entry or learned record that does not fit is skipped and later items are still tried; the 'Also relevant' heading is skipped if it does not fit
- nothing included after the header -> ('', []); the suspect-ask note is pushed only when a suspect record was included

## Terms

context pack, prompt assembly, size limit, truncation, bundle, output builder, included ids, reference context

## Connections

- **Location:** `crates/myrn-engine/src/pack/render.rs:135-174`
- **Signature:** `pub fn pack( cards: &[(String, String)], rest: &[Node], known: &[Node], budget: usize, ) -> (String, Vec<String>)`
- **In:** [[file-crates-myrn-engine-src-pack-render-rs]]
- **Calls:** [[budget-new]], [[budget-push]], [[render-one-line]], [[render-quoted]]
- **Called by:** [[engine-context-excluding]]
