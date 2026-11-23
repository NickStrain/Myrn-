---
id: tests-r
type: summary
status: unverified
title: tests::r
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::tests::r
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::tests::r@e0211f402bb78a6a
map_version: '2'
---
# tests::r

Test helper that runs redact on a &str and returns an owned String.

## Behaviour

- r('x') -> String equal to redact('x')
- Text with no secrets comes back unchanged

## Terms

test helper, redact wrapper, owned result

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:239-241`
- **Signature:** `fn r(s: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Calls:** [[redact-redact]]
- **Called by:** [[tests-clean]]
- **Tests:** `known_token_formats`, `private_key_blocks_including_truncated_ones`, `env_style_assignments`, `credentials_in_urls_and_headers`, `entropy_catches_unknown_random_strings`
