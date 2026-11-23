---
id: tests-sig
type: summary
status: unverified
title: tests::sig
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::tests::sig
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::tests::sig@29dc6525ce550597
map_version: '2'
---
# tests::sig

Test helper that normalizes a string and returns its text form, panicking if normalize finds no error.

## Behaviour

- 'TypeError: x' -> 'TypeError: x'
- 'all good' with no error marker -> panics on unwrap
- 'fatal: object at 0x7ffd... was freed' -> 'error: object at ADDR was freed'

## Terms

test helper, assert helper, unit test shortcut, expected string, text form

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:199-201`
- **Signature:** `fn sig(t: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Calls:** [[signature-normalize]]
- **Tests:** `rust_compiler_and_panics`, `python_tracebacks_use_the_last_line`, `typescript_and_javascript`, `generic_and_addresses`
