---
id: resolver-resolve-path
type: summary
status: unverified
title: Resolver::resolve_path
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_path
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_path@8c01b55b89ce7821
---
# Resolver::resolve_path

Resolves a Rust path call such as Self::f, alias::f, crate::... or Type::f. Non-Rust callers are delegated to resolve_method, and a Type::f whose type lives elsewhere is Inferred only when exactly one candidate matches.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:570-638`
- **Signature:** `fn resolve_path( &self, s: &Sym, name: &str, q: &str, scope: Option<&str>, ) -> Vec<(usize, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-rust-module]], [[sym-parent]], [[sym-callable]], [[binding-external]], [[resolver-rust-path]], [[resolver-lookup]], [[resolver-resolved]], [[resolver-binding]], [[resolver-follow-rust]], [[resolver-resolve-method]], [[resolver-reachable]], [[symbols-last-seg]]
- **Called by:** [[resolver-resolve]]
