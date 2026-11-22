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

Error signature normalization for the distill module: turns raw compiler, panic, traceback, or generic error output into a stable (kind, message) Signature by picking the naming line and scrubbing volatile tokens (paths, line numbers, addresses, timestamps, ids, long numbers), so the same failure produces an identical string across machines. Includes the Signature type with text, word-splitting, and similarity methods, a regex-caching helper, and unit tests.

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[signature-signature]], [[signature-text]], [[signature-words]], [[signature-similarity]], [[signature-re]], [[signature-regex]], [[signature-normalize]], [[signature-pick]], [[signature-scrub]], [[tests-sig]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]]
