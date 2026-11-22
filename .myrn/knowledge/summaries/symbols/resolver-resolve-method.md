---
id: resolver-resolve-method
type: summary
status: unverified
title: Resolver::resolve_method
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_method
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_method@89fc6c431fb08570
---
# Resolver::resolve_method

Resolves a method call on receiver q. self, this and cls walk up enclosing types, receivers rooted in external imports resolve to nothing, and super calls are never resolved. Otherwise it tries import-bound receivers, same-file Class.f, and then an Inferred match by receiver-word type or by a unique, non-common, reachable method name.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:715-824`
- **Signature:** `fn resolve_method(&self, s: &Sym, name: &str, q: &str) -> Vec<(usize, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-sep]], [[sym-parent]], [[sym-callable]], [[resolver-lookup]], [[resolver-resolved]], [[resolver-binding]], [[resolver-exported-from]], [[resolver-reachable]], [[resolver-parent-is-type]], [[resolver-in-file]], [[symbols-last-seg]], [[fts-split-identifier]]
- **Called by:** [[resolver-resolve]], [[resolver-resolve-path]]
