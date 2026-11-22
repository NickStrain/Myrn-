---
id: redact-mark
type: summary
status: unverified
title: redact::mark
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::mark
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::mark@17305eda2d7e5a90
---
# redact::mark

Returns the replacement text `[REDACTED:kind]` for a given kind label such as "secret" or "private-key". Pure formatting, no side effects.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:15-17`
- **Signature:** `pub fn mark(kind: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Called by:** [[redact-redact]]
- **Tests:** `known_token_formats`
