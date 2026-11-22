---
id: tests-sig
type: summary
status: unverified
title: tests::sig
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::tests::sig
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::tests::sig@29dc6525ce550597
---
# tests::sig

Test helper that normalizes a string and returns its `text()`, panicking if normalization yields None.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:199-201`
- **Signature:** `fn sig(t: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Calls:** [[signature-normalize]]
- **Tests:** `rust_compiler_and_panics`, `python_tracebacks_use_the_last_line`, `typescript_and_javascript`, `generic_and_addresses`
