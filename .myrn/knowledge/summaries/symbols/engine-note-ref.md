---
id: engine-note-ref
type: summary
status: unverified
title: Engine::note_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::note_ref
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::note_ref@afb2880943f55a66
map_version: '2'
---
# Engine::note_ref

Finds a note by id with a case-insensitive comparison and returns a Ref whose header is the note id.

## Behaviour

- "Foo" matches a note whose id is "foo"
- no match -> None
- ntype comes from the notes type column

## Terms

record id, note lookup, case-insensitive, note reference

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:88-100`
- **Signature:** `fn note_ref(&self, name: &str) -> Result<Option<Ref>>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Called by:** [[engine-resolve-ref]]
