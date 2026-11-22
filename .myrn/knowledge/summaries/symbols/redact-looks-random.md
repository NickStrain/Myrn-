---
id: redact-looks-random
type: summary
status: unverified
title: redact::looks_random
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::looks_random
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::looks_random@c3df858af11bbfb5
---
# redact::looks_random

Returns true for strings of 20 or more bytes that contain lower, upper and digit characters with at least 5% digits, have a class-switch rate of at least 0.3, and reach 82% of the maximum entropy for their length (capped at 62 symbols). All-hex strings with dashes are rejected first, so hashes and UUIDs are never flagged.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:210-233`
- **Signature:** `pub fn looks_random(s: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Calls:** [[redact-shannon-entropy]]
- **Called by:** [[redact-redact]]
- **Tests:** `entropy_catches_unknown_random_strings`
