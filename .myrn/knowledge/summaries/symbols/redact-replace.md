---
id: redact-replace
type: summary
status: unverified
title: redact::replace
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::replace
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::replace@c3ac61708d4a1224
---
# redact::replace

Applies a regex across the text only if it matches, replacing each match with the closure's output. The closure can return the original match unchanged to leave it alone.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:131-136`
- **Signature:** `fn replace(out: &mut Cow<'_, str>, re: &Regex, f: impl Fn(&Captures) -> String)`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Called by:** [[redact-redact]]
