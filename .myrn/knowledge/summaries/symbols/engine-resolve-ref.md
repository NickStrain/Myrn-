---
id: engine-resolve-ref
type: summary
status: unverified
title: Engine::resolve_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::resolve_ref
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::resolve_ref@cc0dd9c2df93f61f
map_version: '2'
---
# Engine::resolve_ref

Resolves a name from a record to a Ref by trying notes, commit sha, path#qname, and then name lookup, returning a user-facing error string when it cannot.

## Behaviour

- surrounding whitespace and backticks are stripped; empty -> Err "empty reference"
- lookup order: note id (only if notes is true), commit sha, path#qname, then pick(name)
- several candidates -> Err listing up to 5 as name (location) and asking for a qualified name

## Terms

resolve reference, lookup order, ambiguous name, reference error, record target, name lookup

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:136-180`
- **Signature:** `pub(crate) fn resolve_ref( &self, name: &str, notes: bool, ) -> Result<std::result::Result<Ref, String>>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[engine-pick]], [[checks-looks-like-sha]], [[engine-commit-ref]], [[engine-note-ref]], [[engine-code-ref]], [[id-symbol-id]], [[id-test-id]], [[graph-node-graph]]
- **Called by:** [[engine-check-evidence]], [[engine-record]]
