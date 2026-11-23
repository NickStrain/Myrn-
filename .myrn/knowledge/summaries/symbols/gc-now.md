---
id: gc-now
type: summary
status: unverified
title: gc::now
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::now
anchors:
- sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::now@5a0deefa9e5a7ffd
map_version: '2'
---
# gc::now

Returns the current Unix time in whole seconds as i64.

## Behaviour

- Clock after 1970 -> seconds since the epoch
- Clock before the epoch -> 0, which gc uses as the no-timestamp sentinel and makes the cutoff negative, so no log is compacted
- Sub-second part of the time is discarded

## Terms

unix time, epoch seconds, timestamp, wall clock, current time

## Connections

- **Location:** `crates/myrn-engine/src/validity/gc.rs:189-194`
- **Signature:** `fn now() -> i64`
- **In:** [[file-crates-myrn-engine-src-validity-gc-rs]]
- **Called by:** [[engine-gc]]
