---
id: sym-parent
type: summary
status: unverified
title: Sym::parent
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::parent
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::parent@d1f879dcac4c98a5
map_version: '2'
---
# Sym::parent

Returns the enclosing scope's qualified name by cutting base_qname at its last separator for the language.

## Behaviour

- Rust 'a::b::f' -> 'a::b'
- top-level 'f' with no separator -> empty string
- Python 'pkg.A.m' -> 'pkg.A'

## Terms

enclosing scope, containing type, owner, parent scope, outer qualified name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:39-45`
- **Signature:** `fn parent(&self) -> &str`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-sep]], [[sym-base-qname]]
- **Called by:** [[engine-open-located]], [[engine-data-home]], [[git-index-commit]], [[walker-doc]], [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]], [[resolver-parent-is-type]], [[resolver-is-top-level]], [[knowledge-file-history]], [[engine-publish]], [[write-history]], [[write-learning]], [[write]], [[write-packing]], [[write-records]], [[write-sharing]], [[write-summarize]], [[write-validity]], [[write-viz]]
- **Tests:** `shared_records_from_a_teammate_apply_when_their_code_matches`
