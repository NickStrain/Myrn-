---
id: lang-sep
type: summary
status: unverified
title: Lang::sep
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::sep
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::sep@0ed5170637bf4967
---
# Lang::sep

Returns the qualified-name separator: "::" for the Rust family and "." for every other family.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:74-79`
- **Signature:** `pub fn sep(self) -> &'static str`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Calls:** [[lang-family]]
- **Called by:** [[walker-push-def]], [[sym-parent]], [[resolver-resolve-bare]], [[resolver-resolve-method]]
