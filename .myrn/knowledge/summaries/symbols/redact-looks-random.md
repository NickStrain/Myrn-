---
id: redact-looks-random
type: summary
status: unverified
title: redact::looks_random
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::looks_random
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::looks_random@c3df858af11bbfb5
map_version: '2'
---
# redact::looks_random

Decides whether a string looks generated rather than written. It needs at least 20 chars, not all hex or dash, lower, upper and digit characters present, at least 5% digits, a class switch rate of 0.3 or more, and entropy of at least 0.82 times log2(min(len, 62)).

## Behaviour

- 'Xk92hDf7Lq0Zp3Wv8Bn5Rt1Y' -> true
- Any string shorter than 20 chars -> false
- A 40-char hex commit id, or a dashed UUID -> false because all chars are hex or '-'

## Terms

random string, generated token, high entropy, hex exclusion, secret-like value, mixed case and digits

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:210-233`
- **Signature:** `pub fn looks_random(s: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Calls:** [[redact-shannon-entropy]]
- **Called by:** [[redact-redact]]
- **Tests:** `entropy_catches_unknown_random_strings`
