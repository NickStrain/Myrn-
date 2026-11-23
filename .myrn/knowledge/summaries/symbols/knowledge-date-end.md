---
id: knowledge-date-end
type: summary
status: unverified
title: knowledge::date_end
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::date_end
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::date_end@13f1f055b3c9d26c
map_version: '2'
---
# knowledge::date_end

Converts `YYYY-MM-DD` to unix seconds for 23:59:59 UTC on that day, returning None for anything else.

## Behaviour

- `1970-01-01` -> 86399
- `2026-13-01` or `HEAD~1` -> None
- day is only checked to be 1 to 31, so `2026-02-31` is accepted

## Terms

date parsing, end of day, epoch seconds, unix time, UTC, civil date

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:523-542`
- **Signature:** `fn date_end(s: &str) -> Option<i64>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Called by:** [[knowledge-resolve-point]]
- **Tests:** `dates_are_end_of_day_utc`
