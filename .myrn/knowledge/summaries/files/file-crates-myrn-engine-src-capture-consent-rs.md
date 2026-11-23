---
id: file-crates-myrn-engine-src-capture-consent-rs
type: summary
status: unverified
title: crates/myrn-engine/src/capture/consent.rs
source: myrn summarize
about: file:crates/myrn-engine/src/capture/consent.rs
anchors:
- file:crates/myrn-engine/src/capture/consent.rs@4612e590eb6eccf9
---
# crates/myrn-engine/src/capture/consent.rs

Consent gate for session recording: a single predicate, may_record, that decides whether an agent's session may be captured, based on the one-time consent flag and the agent's configured mode in the private state.json.

## Connections

- **Location:** `crates/myrn-engine/src/capture/consent.rs`
- **In:** [[dir-crates-myrn-engine-src-capture]]
- **Defines:** [[consent-may-record]]
- **Imports:** [[file-crates-myrn-store-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-capture-mod-rs]]
