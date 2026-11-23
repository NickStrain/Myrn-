---
id: dir-crates-myrn-tests
type: summary
status: unverified
title: crates/myrn/tests/
source: myrn summarize
about: module:crates/myrn/tests
anchors:
- module:crates/myrn/tests@8165ea5e1e095e93
---
# crates/myrn/tests/

Integration test directory for the myrn CLI. It holds connect.rs, which runs the real myrn binary end to end against a temp git repo: init, connect with consent, hook payloads on stdin, doctor, pull-only Codex setup, and disconnect. Its two tests share helper functions that build the temp repo and invoke myrn.

## Connections

- **In:** [[dir-crates-myrn]]
- **Files:** [[file-crates-myrn-tests-connect-rs]]
