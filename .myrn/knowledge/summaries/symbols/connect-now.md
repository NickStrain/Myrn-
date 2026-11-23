---
id: connect-now
type: summary
status: unverified
title: connect::now
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::now
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::now@c46b3c90f30dd863
map_version: '2'
---
# connect::now

Returns the current Unix time in whole seconds, used for consent and connection timestamps.

## Behaviour

- normal clock -> seconds since 1970-01-01 UTC, sub-second part dropped
- system clock before the Unix epoch -> returns 0

## Terms

timestamp, unix time, epoch seconds, consent time, clock

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:142-147`
- **Signature:** `pub fn now() -> i64`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Called by:** [[connect-run]], [[hook-handle]]
