---
id: consent-may-record
type: summary
status: unverified
title: consent::may_record
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/consent.rs#capture::consent::may_record
anchors:
- sym:crates/myrn-engine/src/capture/consent.rs#capture::consent::may_record@d13f759bfbe4cdb3
map_version: '2'
---
# consent::may_record

Returns true only when recording consent has been given globally and the named agent is registered with mode "hooks".

## Behaviour

- state.capture_consent false -> false, regardless of agent
- agent name not present in state.agents -> false (lookup yields None, is_some_and returns false)
- consent true and agent mode "hooks" -> true; any other mode string such as "manual" -> false

## Terms

recording consent, capture permission, session logging gate, opt-in check, hooks mode, privacy gate

## Connections

- **Location:** `crates/myrn-engine/src/capture/consent.rs:7-9`
- **Signature:** `pub fn may_record(state: &State, agent: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-consent-rs]]
- **Called by:** [[engine-capture]]
