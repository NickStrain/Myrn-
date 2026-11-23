---
id: lang-family
type: summary
status: unverified
title: Lang::family
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::family
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::family@b1499aa25cc1c6a4
map_version: '2'
---
# Lang::family

Groups each language into a Family used for resolution rules. JavaScript, TypeScript and Tsx all map to Family::Js.

## Behaviour

- Rust -> Family::Rust
- Python -> Family::Python
- JavaScript, TypeScript, Tsx -> Family::Js

## Terms

resolution family, language group, shared rules, rule family, family

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:65-71`
- **Signature:** `pub fn family(self) -> Family`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[lang-sep]], [[lang-module-prefix]], [[walker-rust-mod-path]], [[walker-note-local]], [[walker-doc]], [[walker-classify]], [[resolver-new]], [[resolver-add-import]], [[resolver-lookup]], [[resolver-lookup-in-crate]], [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]], [[resolver-parent-is-type]], [[resolver-is-top-level]]
