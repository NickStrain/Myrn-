---
id: redact-harmless-value
type: summary
status: unverified
title: redact::harmless_value
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::harmless_value
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::harmless_value@a9dde069e4d4fb24
---
# redact::harmless_value

Returns true for values next to a sensitive name that should not be redacted: empty, already redacted, booleans, null-like words, auth scheme words, numbers, environment references ($VAR, %VAR%), angle or double-brace placeholders, and runs of * x X or dots. Trims surrounding quotes before checking.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:140-164`
- **Signature:** `fn harmless_value(v: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Called by:** [[redact-redact]]
