---
id: resolver-resolve-path
type: summary
status: unverified
title: Resolver::resolve_path
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_path
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_path@8c01b55b89ce7821
map_version: '2'
---
# Resolver::resolve_path

Resolves a path-qualified call. Rust paths go through Self, import aliases, module paths, then a type-name match; other languages are sent to resolve_method.

## Behaviour

- 'Self::new' inside impl Session -> Session::new in the same type, Resolved
- non-Rust source -> resolve_method
- type-name fallback: unique match in the same crate, else unique match overall, else empty

## Terms

path call, associated function, type name match, Self call, static call, scoped call

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:570-638`
- **Signature:** `fn resolve_path( &self, s: &Sym, name: &str, q: &str, scope: Option<&str>, ) -> Vec<(usize, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-rust-module]], [[sym-parent]], [[sym-callable]], [[binding-external]], [[resolver-rust-path]], [[resolver-lookup]], [[resolver-resolved]], [[resolver-binding]], [[resolver-follow-rust]], [[resolver-resolve-method]], [[resolver-reachable]], [[symbols-last-seg]]
- **Called by:** [[resolver-resolve]]
