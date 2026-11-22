---
id: contradiction-words
type: summary
status: unverified
title: contradiction::words
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/contradiction.rs#distill::contradiction::words
anchors:
- sym:crates/myrn-engine/src/distill/contradiction.rs#distill::contradiction::words@f2c80903d051530e
---
# contradiction::words

Lowercases the text, removes apostrophes, and splits on anything that is not alphanumeric or underscore. It returns the set of tokens longer than 2 characters that are not negation words, plus a count of negation words from the NEGATIONS list. Tokens of 2 or fewer characters are dropped.

## Connections

- **Location:** `crates/myrn-engine/src/distill/contradiction.rs:21-37`
- **Signature:** `fn words(text: &str) -> (HashSet<String>, usize)`
- **In:** [[file-crates-myrn-engine-src-distill-contradiction-rs]]
- **Called by:** [[contradiction-opposite]]
