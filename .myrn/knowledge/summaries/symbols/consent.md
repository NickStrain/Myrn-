---
id: consent
type: summary
status: unverified
title: consent
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#consent
anchors:
- sym:crates/myrn-engine/tests/records.rs#consent@70c8e0cdb3f4f988
---
# consent

Sets capture_consent to true and registers a 'claude' agent in hooks mode in the engine state, then saves it so events are captured.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:492-505`
- **Signature:** `fn consent(e: &Engine)`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[engine-state]], [[engine-save-state]]
- **Tests:** `shell_edits_are_captured_and_lead_to_a_record_request`
