---
id: history-days-to-date
type: summary
status: unverified
title: history::days_to_date
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/history.rs#tools::history::days_to_date
anchors:
- sym:crates/myrn-mcp/src/tools/history.rs#tools::history::days_to_date@e3aada43de9cbac3
map_version: '2'
---
# history::days_to_date

Converts a count of days since 1970-01-01 into a zero-padded YYYY-MM-DD string using the civil-from-days algorithm.

## Behaviour

- 0 -> 1970-01-01
- 20773 -> 2026-11-16 (asserted in the unit test)
- -1 -> 1969-12-31; negative day counts are handled through div_euclid-based era arithmetic

## Terms

epoch day, unix days, days since epoch, civil date, Gregorian date, date formatting, leap year, date conversion

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs:49-60`
- **Signature:** `fn days_to_date(z: i64) -> String`
- **In:** [[file-crates-myrn-mcp-src-tools-history-rs]]
- **Called by:** [[history-render]]
- **Tests:** `dates`
