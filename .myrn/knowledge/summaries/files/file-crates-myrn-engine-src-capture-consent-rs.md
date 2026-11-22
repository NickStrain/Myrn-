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

Consent state for session recording: a single predicate, may_record, that gates whether an agent's session may be captured. Consent is given once in `myrn connect` and stored in the private state.json; nothing from a session is written without it.

## Connections

- **Location:** `crates/myrn-engine/src/capture/consent.rs`
- **In:** [[dir-crates-myrn-engine-src-capture]]
- **Defines:** [[consent-may-record]]
- **Imports:** [[file-crates-myrn-store-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-capture-mod-rs]]
