---
id: sharing-ymd
type: summary
status: unverified
title: sharing::ymd
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::ymd
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::ymd@a493b4f583bccb64
map_version: '2'
---
# sharing::ymd

Converts unix seconds to a UTC YYYY-MM-DD string using civil-from-days arithmetic. Floor division is used, so partial days are dropped and negative values give dates before 1970.

## Behaviour

- 0 -> '1970-01-01'
- 1_794_787_200 + 3_660 -> '2026-11-16' (as in the test)
- negative seconds -> dates before 1970, floored rather than truncated

## Terms

unix time to date, epoch seconds, civil date, date format, day number, date conversion

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:303-314`
- **Signature:** `pub fn ymd(t: i64) -> String`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
- **Called by:** [[sharing-today]]
- **Tests:** `dates`
