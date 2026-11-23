---
id: sym-base-qname
type: summary
status: unverified
title: Sym::base_qname
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::base_qname
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::base_qname@f54a738cc1955dcc
map_version: '2'
---
# Sym::base_qname

Returns the qualified name with any trailing '#n' collision suffix removed.

## Behaviour

- 'pkg.f#3' -> 'pkg.f'
- 'pkg.f#x' -> 'pkg.f#x' unchanged, since the suffix is not all digits
- 'f' -> 'f'

## Terms

collision suffix, hash suffix, base name, unsuffixed name, disambiguated name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:34-36`
- **Signature:** `fn base_qname(&self) -> &str`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[symbols-strip-suffix]]
- **Called by:** [[sym-parent]], [[resolver-new]], [[resolver-resolve-bare]]
