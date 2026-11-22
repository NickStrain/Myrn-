---
id: engine-resolve-ref
type: summary
status: unverified
title: Engine::resolve_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::resolve_ref
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::resolve_ref@cc0dd9c2df93f61f
---
# Engine::resolve_ref

Trims whitespace and backticks, then tries in order: note id (only if notes is true), commit sha (if sha-shaped), exact symbol or test id from a path#qname form, and fuzzy symbol lookup. The outer Result carries database errors, and the inner Err is a user-facing message for empty, ambiguous (listing up to 5 candidates), or unmatched names.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:136-180`
- **Signature:** `pub(crate) fn resolve_ref( &self, name: &str, notes: bool, ) -> Result<std::result::Result<Ref, String>>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[engine-pick]], [[checks-looks-like-sha]], [[engine-commit-ref]], [[engine-note-ref]], [[engine-code-ref]], [[id-symbol-id]], [[id-test-id]], [[graph-node-graph]]
- **Called by:** [[engine-check-evidence]], [[engine-record]]
