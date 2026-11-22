---
id: signature-re
type: summary
status: unverified
title: signature::re
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::re
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::re@2277689a52151bb5
---
# signature::re

Helper that compiles a pattern into a caller-provided `OnceLock` cell, panicking on an invalid pattern; used by the `regex!` macro.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:56-58`
- **Signature:** `fn re(cell: &'static OnceLock<Regex>, pat: &str) -> &'static Regex`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
