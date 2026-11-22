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

Secret redaction for captured text, run before anything is written. It applies, in order: whole private-key blocks, known token formats (AWS, GitHub, GitLab, Slack, JWT, API keys, Stripe, Google, npm, PyPI), credentials in URLs and auth headers, sensitive KEY=VALUE and --flag VALUE assignments, and finally a Shannon-entropy check for unknown random strings, with hex, paths and identifiers exempted.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs`
- **In:** [[dir-crates-myrn-engine-src-capture]]
- **Defines:** [[redact-mark]], [[redact-rule]], [[redact-rule-redact]], [[redact-redact]], [[redact-replace]], [[redact-harmless-value]], [[redact-shannon-entropy]], [[redact-class]], [[redact-class-redact]], [[redact-looks-random]], [[tests-r]], [[tests-clean]]
- **Imported by:** [[file-crates-myrn-engine-src-capture-mod-rs]]
