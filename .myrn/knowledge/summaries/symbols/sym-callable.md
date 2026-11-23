---
id: sym-callable
type: summary
status: unverified
title: Sym::callable
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::callable
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::callable@678d80abae931bb3
map_version: '2'
---
# Sym::callable

Reports whether the symbol can be the target of a call: not a test and kind is function, method, class or struct.

## Behaviour

- kind 'function' and not a test -> true
- kind 'test' -> false
- any other kind -> false

## Terms

callable, call target, invocable, function or method, non-test

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:47-53`
- **Signature:** `fn callable(&self) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[kind-as-str]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]]
