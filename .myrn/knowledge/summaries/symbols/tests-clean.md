---
id: tests-clean
type: summary
status: unverified
title: tests::clean
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::tests::clean
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::tests::clean@ab9ccf1a5636c69f
map_version: '2'
---
# tests::clean

Test helper that asserts redact leaves the input unchanged, and includes the input in the failure message.

## Behaviour

- clean('max_tokens=1500') passes
- clean('AKIAIOSFODNN7EXAMPLE') fails because the input is redacted

## Terms

unchanged assertion, no-op check, false positive guard, should be left alone

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:243-245`
- **Signature:** `fn clean(s: &str)`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Calls:** [[tests-r]]
- **Tests:** `private_key_blocks_including_truncated_ones`, `harmless_text_is_left_alone`
