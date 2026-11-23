---
id: signature-regex
type: summary
status: unverified
title: signature::regex
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::regex
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::regex@ff5a6c399d1b645d
map_version: '2'
---
# signature::regex

Declares a static once-cell at each expansion site and returns the cached compiled Regex for the given pattern literal.

## Behaviour

- each call site has its own cache, so the same pattern used at two sites compiles twice
- pattern is compiled at first use of that site and reused on later calls
- an invalid pattern panics at first use, not at build time

## Terms

compile cache, static regex, once per site, regex literal, lazy static, pattern literal

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:60-65`
- **Signature:** `macro_rules! regex`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
