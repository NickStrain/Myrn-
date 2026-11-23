---
id: file-crates-myrn-tests-connect-rs
type: summary
status: unverified
title: crates/myrn/tests/connect.rs
source: myrn summarize
about: file:crates/myrn/tests/connect.rs
anchors:
- file:crates/myrn/tests/connect.rs@88847bae83f57d3a
---
# crates/myrn/tests/connect.rs

Integration test that drives the real myrn binary end to end: init, connect with consent, hook payloads on stdin, doctor, pull-only Codex setup, and disconnect. Its two tests share the helpers below, which build a temp git repo and run myrn against it.

## Connections

- **Location:** `crates/myrn/tests/connect.rs`
- **In:** [[dir-crates-myrn-tests]]
- **Defines:** [[env]], [[setup-connect]], [[env-cmd]], [[env-myrn]], [[env-hook]], [[env-settings]], [[text]]
