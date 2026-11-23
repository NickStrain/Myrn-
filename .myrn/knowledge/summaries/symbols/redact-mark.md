---
id: redact-mark
type: summary
status: unverified
title: redact::mark
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::mark
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::mark@17305eda2d7e5a90
map_version: '2'
---
# redact::mark

Builds the replacement marker [REDACTED:kind] for a given kind label. It does no validation or escaping of the label.

## Behaviour

- mark('api-key') -> '[REDACTED:api-key]'
- Any string is wrapped as given, including an empty one, which yields '[REDACTED:]'

## Terms

replacement marker, placeholder text, redaction label, masking string

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:15-17`
- **Signature:** `pub fn mark(kind: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Called by:** [[redact-redact]]
- **Tests:** `known_token_formats`
