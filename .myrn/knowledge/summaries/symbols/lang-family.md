---
id: lang-family
type: summary
status: unverified
title: Lang::family
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::family
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::family@b1499aa25cc1c6a4
---
# Lang::family

Maps Rust to Family::Rust and Python to Family::Python. JavaScript, TypeScript and Tsx all map to Family::Js.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:65-71`
- **Signature:** `pub fn family(self) -> Family`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[lang-sep]], [[lang-module-prefix]], [[walker-rust-mod-path]], [[walker-note-local]], [[walker-doc]], [[walker-classify]], [[resolver-new]], [[resolver-add-import]], [[resolver-lookup]], [[resolver-lookup-in-crate]], [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]], [[resolver-parent-is-type]], [[resolver-is-top-level]]
