---
id: redact-redact
type: summary
status: unverified
title: redact::redact
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::redact
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::redact@e835821d9e0f7c7b
---
# redact::redact

Takes text and returns it with secrets replaced, as a Cow that borrows the input when nothing matches. Runs the stages in fixed order (tokens, URL credentials, auth headers, assignments and flags, then entropy), so later stages see earlier replacements, and skips values already marked as redacted.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:95-129`
- **Signature:** `pub fn redact(text: &str) -> Cow<'_, str>`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Calls:** [[kind-as-str]], [[redact-mark]], [[redact-replace]], [[redact-harmless-value]], [[redact-looks-random]]
- **Called by:** [[capture-to-entry]], [[capture-clean]], [[tests-r]]
