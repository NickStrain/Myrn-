---
id: redact-shannon-entropy
type: summary
status: unverified
title: redact::shannon_entropy
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::shannon_entropy
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::shannon_entropy@7bc2ceb524fa2087
map_version: '2'
---
# redact::shannon_entropy

Computes Shannon entropy of the string in bits per byte. The doc comment says per character, but the code counts bytes, so multi-byte UTF-8 characters count once per byte.

## Behaviour

- Empty string -> 0.0
- 'aaaa' -> 0.0 (one symbol, no uncertainty)
- 'abcd' -> 2.0 (four equally likely bytes)

## Terms

entropy, randomness measure, bits per byte, information content, character spread

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:167-185`
- **Signature:** `pub fn shannon_entropy(s: &str) -> f64`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Called by:** [[redact-looks-random]]
- **Tests:** `entropy_values`
