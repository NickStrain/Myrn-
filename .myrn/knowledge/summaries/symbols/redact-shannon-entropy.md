---
id: redact-shannon-entropy
type: summary
status: unverified
title: redact::shannon_entropy
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::shannon_entropy
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::shannon_entropy@7bc2ceb524fa2087
---
# redact::shannon_entropy

Returns the Shannon entropy of the string in bits per byte, computed over byte frequencies. Returns 0.0 for an empty string.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:167-185`
- **Signature:** `pub fn shannon_entropy(s: &str) -> f64`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Called by:** [[redact-looks-random]]
- **Tests:** `entropy_values`
