---
id: log-datetime
type: summary
status: unverified
title: log::datetime
source: myrn summarize
about: sym:crates/myrn/src/cmd/log.rs#cmd::log::datetime
anchors:
- sym:crates/myrn/src/cmd/log.rs#cmd::log::datetime@14b3ad702b043b70
map_version: '2'
---
# log::datetime

Converts Unix seconds to a UTC string 'YYYY-MM-DD HH:MM' using a civil-date conversion that floors for negative values. Seconds are dropped.

## Behaviour

- 0 -> '1970-01-01 00:00'
- 1_794_787_200 + 3_660 -> '2026-11-16 01:01' (the unit test case)
- -1 -> '1969-12-31 23:59' because div_euclid and rem_euclid floor before the epoch

## Terms

timestamp, unix time, epoch seconds, utc date, date string, civil date, time formatting

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs:208-224`
- **Signature:** `pub(crate) fn datetime(t: i64) -> String`
- **In:** [[file-crates-myrn-src-cmd-log-rs]]
- **Called by:** [[log-run]], [[log-render-one]], [[trace-run]]
- **Tests:** `datetimes`
