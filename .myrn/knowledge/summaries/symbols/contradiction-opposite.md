---
id: contradiction-opposite
type: summary
status: unverified
title: contradiction::opposite
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/contradiction.rs#distill::contradiction::opposite
anchors:
- sym:crates/myrn-engine/src/distill/contradiction.rs#distill::contradiction::opposite@adc0637b5c391113
---
# contradiction::opposite

Returns true only if both texts have content words and an unequal count of negation words (odd vs even parity, so a double negation cancels out). It also requires the Jaccard similarity of their content-word sets, intersection over union, to be at least SAME_CLAIM (0.6).

## Connections

- **Location:** `crates/myrn-engine/src/distill/contradiction.rs:40-48`
- **Signature:** `pub fn opposite(a: &str, b: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-distill-contradiction-rs]]
- **Calls:** [[contradiction-words]]
- **Called by:** [[contradiction-find]]
- **Tests:** `negated_claims_conflict`
