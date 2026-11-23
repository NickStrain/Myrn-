---
id: sharing-today
type: summary
status: unverified
title: sharing::today
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::today
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::today@351ac3f095ad46b4
map_version: '2'
---
# sharing::today

Returns the current UTC date as YYYY-MM-DD, using ymd on the current unix time.

## Behaviour

- clock before the unix epoch -> uses 0 -> '1970-01-01'
- output is always UTC, never local time

## Terms

date, current date, published date, utc date, today, timestamp

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:294-300`
- **Signature:** `pub fn today() -> String`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
- **Calls:** [[sharing-ymd]]
- **Called by:** [[engine-record]], [[engine-publish-plan]]
