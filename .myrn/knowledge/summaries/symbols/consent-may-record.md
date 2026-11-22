---
id: consent-may-record
type: summary
status: unverified
title: consent::may_record
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/consent.rs#capture::consent::may_record
anchors:
- sym:crates/myrn-engine/src/capture/consent.rs#capture::consent::may_record@d13f759bfbe4cdb3
---
# consent::may_record

Takes &State and an agent name and returns true only if state.capture_consent is set and that agent's entry in state.agents exists with mode equal to "hooks"; returns false otherwise, with no side effects. Both conditions must hold, so an agent with any other mode is never recorded even when consent is given.

## Connections

- **Location:** `crates/myrn-engine/src/capture/consent.rs:7-9`
- **Signature:** `pub fn may_record(state: &State, agent: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-consent-rs]]
- **Called by:** [[engine-capture]]
