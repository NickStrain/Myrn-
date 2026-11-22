---
id: signature-regex
type: summary
status: unverified
title: signature::regex
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::regex
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::regex@ff5a6c399d1b645d
---
# signature::regex

Macro that declares a function-local static `OnceLock<Regex>` and returns the compiled regex, so each call site compiles its pattern only once.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:60-65`
- **Signature:** `macro_rules! regex`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
