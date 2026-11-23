---
id: claims-opposite
type: summary
status: unverified
title: claims::opposite
source: myrn summarize
about: sym:crates/myrn-store/src/notes/claims.rs#notes::claims::opposite
anchors:
- sym:crates/myrn-store/src/notes/claims.rs#notes::claims::opposite@adc0637b5c391113
map_version: '2'
---
# claims::opposite

Returns true only when the two statements have different negation-count parity and at least 60% word overlap (common words divided by union of words). Negation words are excluded from the overlap.

## Behaviour

- 'Call X before Y.' vs 'Never call X before Y.' -> true: counts 0 and 1 differ in parity, and the word sets overlap heavily
- 'The refresh window is configurable.' vs 'The refresh window isn't configurable.' -> true: 'isnt' is a negation, so the word sets are identical
- Both sides with equal negation parity (both 0, or 1 and 2) -> false, even when the wording is nearly identical

## Terms

contradiction, conflict, contradicting statements, negated duplicate, inconsistent claims, similarity overlap, Jaccard similarity, same subject

## Connections

- **Location:** `crates/myrn-store/src/notes/claims.rs:36-44`
- **Signature:** `pub fn opposite(a: &str, b: &str) -> bool`
- **In:** [[file-crates-myrn-store-src-notes-claims-rs]]
- **Calls:** [[claims-words]]
- **Called by:** [[contradiction-find]], [[sync-shared-contradictions]]
- **Tests:** `negated_claims_conflict`
