---
id: file-crates-myrn-engine-src-capture-redact-rs
type: summary
status: unverified
title: crates/myrn-engine/src/capture/redact.rs
source: myrn summarize
about: file:crates/myrn-engine/src/capture/redact.rs
anchors:
- file:crates/myrn-engine/src/capture/redact.rs@10f493373a9e89a3
---
# crates/myrn-engine/src/capture/redact.rs

Secret redaction for captured text, applied before anything is written. It runs pattern rules for private-key blocks, known token formats, URL and header credentials, sensitive KEY=VALUE and --flag values, then an entropy check for random strings. The public entry point is redact, and the file ends with unit tests.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs`
- **In:** [[dir-crates-myrn-engine-src-capture]]
- **Defines:** [[redact-mark]], [[redact-rule]], [[redact-rule-redact]], [[redact-redact]], [[redact-replace]], [[redact-harmless-value]], [[redact-shannon-entropy]], [[redact-class]], [[redact-class-redact]], [[redact-looks-random]], [[tests-r]], [[tests-clean]]
- **Imported by:** [[file-crates-myrn-engine-src-capture-mod-rs]]
