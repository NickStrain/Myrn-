---
id: file-crates-myrn-engine-src-distill-signature-rs
type: summary
status: unverified
title: crates/myrn-engine/src/distill/signature.rs
source: myrn summarize
about: file:crates/myrn-engine/src/distill/signature.rs
anchors:
- file:crates/myrn-engine/src/distill/signature.rs@7de23a68df5dddd2
---
# crates/myrn-engine/src/distill/signature.rs

Turns raw error output from compilers, panics, tracebacks and test runners into a Signature (kind plus scrubbed message), so the same failure from another machine or day compares equal. Main parts: the Signature type with text, words and similarity helpers, normalize as the entry point, pick which finds the error line, scrub which strips volatile parts, and unit tests.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[signature-signature]], [[signature-text]], [[signature-words]], [[signature-similarity]], [[signature-re]], [[signature-regex]], [[signature-normalize]], [[signature-pick]], [[signature-scrub]], [[tests-sig]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]]
