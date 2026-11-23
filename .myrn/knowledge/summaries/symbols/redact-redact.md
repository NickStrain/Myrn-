---
id: redact-redact
type: summary
status: unverified
title: redact::redact
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::redact
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::redact@e835821d9e0f7c7b
map_version: '2'
---
# redact::redact

Returns the text with every detected secret replaced by a marker. It borrows the input when nothing matches and applies the passes in a fixed order: tokens, URL passwords, auth headers, assignments and flags, then entropy.

## Behaviour

- Text with no secrets -> Cow::Borrowed, with no allocation
- 'AKIAIOSFODNN7EXAMPLE' -> '[REDACTED:aws-access-key]'
- 'https://bob:pw@host' -> 'https://bob:[REDACTED:password]@host', keeping the user name

## Terms

scrub secrets, sanitize output, mask credentials, secret detection, pre-write filter

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:95-129`
- **Signature:** `pub fn redact(text: &str) -> Cow<'_, str>`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Calls:** [[kind-as-str]], [[redact-mark]], [[redact-replace]], [[redact-harmless-value]], [[redact-looks-random]]
- **Called by:** [[capture-to-entry]], [[capture-clean]], [[tests-r]]
