---
id: engine-note-ref
type: summary
status: unverified
title: Engine::note_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::note_ref
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::note_ref@afb2880943f55a66
---
# Engine::note_ref

Looks up a note by id case-insensitively and returns a Ref whose header is the note id, or None if no note matches.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:88-100`
- **Signature:** `fn note_ref(&self, name: &str) -> Result<Option<Ref>>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Called by:** [[engine-resolve-ref]]
