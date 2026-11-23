---
id: signature-re
type: summary
status: unverified
title: signature::re
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::re
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::re@2277689a52151bb5
map_version: '2'
---
# signature::re

Returns the regex cached in the given once-cell, compiling pat on the first call and panicking if pat is not a valid regex.

## Behaviour

- first call with an empty cell -> compiles pat, stores it, returns it
- later calls with the same cell -> returns the cached regex and ignores pat
- invalid pattern -> panics with 'valid regex'

## Terms

regex cache, compile once, lazy regex, memoized pattern, static regex

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:56-58`
- **Signature:** `fn re(cell: &'static OnceLock<Regex>, pat: &str) -> &'static Regex`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
