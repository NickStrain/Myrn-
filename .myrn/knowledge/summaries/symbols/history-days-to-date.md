---
id: history-days-to-date
type: summary
status: unverified
title: history::days_to_date
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/history.rs#tools::history::days_to_date
anchors:
- sym:crates/myrn-mcp/src/tools/history.rs#tools::history::days_to_date@e3aada43de9cbac3
---
# history::days_to_date

Converts days since 1970-01-01 (i64, negative allowed) to a zero-padded `YYYY-MM-DD` string using the proleptic Gregorian civil-from-days algorithm, with no timezone handling. A unit test checks day 0 and day 20773.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs:49-60`
- **Signature:** `fn days_to_date(z: i64) -> String`
- **In:** [[file-crates-myrn-mcp-src-tools-history-rs]]
- **Called by:** [[history-render]]
- **Tests:** `dates`
