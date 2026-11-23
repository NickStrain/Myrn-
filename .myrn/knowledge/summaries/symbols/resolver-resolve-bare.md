---
id: resolver-resolve-bare
type: summary
status: unverified
title: Resolver::resolve_bare
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_bare
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve_bare@6eeaba0f4e37c363
map_version: '2'
---
# Resolver::resolve_bare

Resolves an unqualified call name: first through imports, then same-file callables, then wildcard imports, then a unique same-family function that is reachable and not a common name.

## Behaviour

- name imported from an external module -> empty
- local non-method function with that name in the file -> Resolved, nearest parent chosen by common prefix
- unique non-common function elsewhere that is reachable -> Inferred

## Terms

bare call, unqualified call, local call, wildcard import, inferred call, common name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:495-568`
- **Signature:** `fn resolve_bare(&self, s: &Sym, name: &str) -> Vec<(usize, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-sep]], [[sym-base-qname]], [[sym-parent]], [[sym-callable]], [[binding-external]], [[resolver-lookup]], [[resolver-resolved]], [[resolver-binding]], [[resolver-reexported]], [[resolver-reachable]], [[resolver-in-file]], [[symbols-last-seg]], [[symbols-common-prefix]]
- **Called by:** [[resolver-resolve]]
