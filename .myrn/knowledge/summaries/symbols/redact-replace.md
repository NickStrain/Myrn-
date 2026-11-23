---
id: redact-replace
type: summary
status: unverified
title: redact::replace
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::replace
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::replace@c3ac61708d4a1224
map_version: '2'
---
# redact::replace

Applies a regex to the Cow in place, replacing each match with the closure result. The string is rebuilt only when the regex matches.

## Behaviour

- No match -> the Cow stays Borrowed and nothing is copied
- The closure gets Captures: c[0] is the whole match and c[1], c[2] are the groups
- Uses replace_all, so every match is rewritten

## Terms

in-place substitution, capture rewrite, regex replace, conditional rewrite

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:131-136`
- **Signature:** `fn replace(out: &mut Cow<'_, str>, re: &Regex, f: impl Fn(&Captures) -> String)`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Called by:** [[redact-redact]]
