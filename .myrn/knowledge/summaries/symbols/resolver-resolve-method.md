---
id: resolver-resolve-method
type: summary
status: unverified
title: Resolver::resolve_method
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_method
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_method@89fc6c431fb08570
map_version: '2'
---
# Resolver::resolve_method

Resolves a method call on a receiver text: self/this/cls walk up enclosing types, import receivers and same-file Class.f come next, then type-name hints, then a unique non-common method.

## Behaviour

- 'self.f()' inside a method -> the nearest enclosing type's f, Resolved
- 'super.f()' or 'super().f()' -> empty, parent class unknown
- receiver rooted in an external import such as 'Command::new(p).output()' -> empty

## Terms

method call, receiver, self call, super call, type hint, inferred method, receiver name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:715-824`
- **Signature:** `fn resolve_method(&self, s: &Sym, name: &str, q: &str) -> Vec<(usize, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-sep]], [[sym-parent]], [[sym-callable]], [[resolver-lookup]], [[resolver-resolved]], [[resolver-binding]], [[resolver-exported-from]], [[resolver-reachable]], [[resolver-parent-is-type]], [[resolver-in-file]], [[symbols-last-seg]], [[fts-split-identifier]]
- **Called by:** [[resolver-resolve]], [[resolver-resolve-path]]
