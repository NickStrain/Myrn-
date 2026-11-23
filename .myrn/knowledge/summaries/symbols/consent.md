---
id: consent
type: summary
status: unverified
title: consent
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#consent
anchors:
- sym:crates/myrn-engine/tests/records.rs#consent@70c8e0cdb3f4f988
map_version: '2'
---
# consent

Turns on capture consent in the engine state and registers a 'claude' agent in hooks mode with no written or created files, then saves the state.

## Behaviour

- any engine -> state with capture_consent true and claude in hooks mode
- existing claude entry -> replaced, since HashMap insert overwrites
- save failure -> panic

## Terms

opt in, enable capture, hooks mode, agent connection, grant permission, state setup

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:492-505`
- **Signature:** `fn consent(e: &Engine)`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[engine-state]], [[engine-save-state]]
- **Tests:** `shell_edits_are_captured_and_lead_to_a_record_request`
