---
id: log-datetime
type: summary
status: unverified
title: log::datetime
source: myrn summarize
about: sym:crates/myrn/src/cmd/log.rs#cmd::log::datetime
anchors:
- sym:crates/myrn/src/cmd/log.rs#cmd::log::datetime@c77e8cb9d2448231
---
# log::datetime

Converts Unix seconds to a UTC 'YYYY-MM-DD HH:MM' string using a days-from-epoch civil-date conversion that handles negative timestamps. render_one slices the time-of-day from byte offset 11, so the fixed format is load-bearing.

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs:128-144`
- **Signature:** `fn datetime(t: i64) -> String`
- **In:** [[file-crates-myrn-src-cmd-log-rs]]
- **Called by:** [[log-run]], [[log-render-one]]
- **Tests:** `datetimes`
