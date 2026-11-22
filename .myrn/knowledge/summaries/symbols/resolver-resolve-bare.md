---
id: resolver-resolve-bare
type: summary
status: unverified
title: Resolver::resolve_bare
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_bare
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_bare@6eeaba0f4e37c363
---
# Resolver::resolve_bare

Resolves a bare call by trying, in order: the imported binding (including re-exports and JS default imports), a non-method callable in the same file, wildcard imports, and finally a unique non-common free function in the same language family that the caller can reach. Only the JS default-import match and the final unique-name match are Inferred.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:495-568`
- **Signature:** `fn resolve_bare(&self, s: &Sym, name: &str) -> Vec<(usize, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-sep]], [[sym-base-qname]], [[sym-parent]], [[sym-callable]], [[binding-external]], [[resolver-lookup]], [[resolver-resolved]], [[resolver-binding]], [[resolver-reexported]], [[resolver-reachable]], [[resolver-in-file]], [[symbols-last-seg]], [[symbols-common-prefix]]
- **Called by:** [[resolver-resolve]]
