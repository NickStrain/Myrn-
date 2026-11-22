---
id: cards-tests-summary
type: summary
status: unverified
title: cards::tests_summary
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::tests_summary
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::tests_summary@72cbfdc37a6bd4ca
---
# cards::tests_summary

Returns 'none found' for an empty slice. Otherwise it groups tests by the part of location before the first colon, prints the total and the top three files by count (ties sorted by path), and appends '… N more files' when there are more than three.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:190-214`
- **Signature:** `pub fn tests_summary(tests: &[Link]) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Called by:** [[symbolcard-render-compact]]
