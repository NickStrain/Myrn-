---
id: helper-token
type: summary
status: unverified
title: helper::token
source: myrn summarize
about: sym:crates/myrn-embed/src/helper.rs#helper::token
anchors:
- sym:crates/myrn-embed/src/helper.rs#helper::token@3cccbae65d1e98f3
map_version: '2'
---
# helper::token

Generates a 64-character lowercase hex secret for this run by hashing a counter, the nanosecond clock, and the process id, four times. It is the shared secret that gates requests.

## Behaviour

- Always returns 64 hex characters, four 16-digit blocks.
- Each block uses a fresh RandomState, so values differ between calls and processes.
- Clock before UNIX epoch -> the time term is 0 and generation continues.

## Terms

secret, random key, access key, nonce, auth key, per-run secret, hash-based randomness

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs:111-128`
- **Signature:** `fn token() -> String`
- **In:** [[file-crates-myrn-embed-src-helper-rs]]
- **Called by:** [[helper-serve]]
