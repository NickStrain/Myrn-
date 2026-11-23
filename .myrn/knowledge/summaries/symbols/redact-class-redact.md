---
id: redact-class-redact
type: summary
status: unverified
title: redact::class
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::class
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::class@7c60459fc16e9ba4
map_version: '2'
---
# redact::class

Maps a char to its Class. Only ASCII letters and digits get their own class.

## Behaviour

- 'q' -> Lower, 'Q' -> Upper, '7' -> Digit
- '_', '-', '/' -> Other
- Non-ASCII letters such as 'é' -> Other

## Terms

character classification, char kind, ascii bucket, case mapping

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:195-205`
- **Signature:** `fn class(c: char) -> Class`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
