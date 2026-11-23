---
id: signature-pick
type: summary
status: unverified
title: signature::pick
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::pick
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::pick@f9321ae34da11719
map_version: '2'
---
# signature::pick

Finds the line naming the error and returns its raw (kind, message) without scrubbing, trying rustc, tsc, panic, typed exception, generic prefix, then any line containing error or FAILED, in that order.

## Behaviour

- 'error[E0308]: mismatched types' -> ('E0308', 'mismatched types')
- 'thread main panicked at 'boom', src/lib.rs:3:5' -> ('panic', 'boom'); a new-style panic with no inline text takes the next line as message
- text containing a 'Traceback (most recent call last)' header is scanned bottom-up, so the last 'KeyError: x' line wins; 'pkg.errors.AuthError: bad' -> kind 'AuthError'

## Terms

error line, error kind, panic message, traceback, exception class, precedence, message extraction, error detection

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:85-150`
- **Signature:** `fn pick(lines: &[&str]) -> Option<(String, String)>`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Called by:** [[signature-normalize]]
