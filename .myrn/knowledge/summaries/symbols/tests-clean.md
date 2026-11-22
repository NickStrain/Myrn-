---
id: tests-clean
type: summary
status: unverified
title: tests::clean
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::tests::clean
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::tests::clean@ab9ccf1a5636c69f
---
# tests::clean

Test helper asserting that `redact` leaves the input unchanged, with the input shown in the failure message.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:243-245`
- **Signature:** `fn clean(s: &str)`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Calls:** [[tests-r]]
- **Tests:** `private_key_blocks_including_truncated_ones`, `harmless_text_is_left_alone`
