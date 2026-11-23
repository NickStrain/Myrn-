---
id: report-diff-cell
type: summary
status: unverified
title: report::diff_cell
source: myrn summarize
about: sym:bench/src/report.rs#report::diff_cell
anchors:
- sym:bench/src/report.rs#report::diff_cell@773ae734725e14bf
map_version: '2'
---
# report::diff_cell

Formats a Gate's token difference as 'mean ± interval (signed percent)' in thousands for the paired comparison table.

## Behaviour

- Mean and interval both present -> the ± form with the percent in parentheses
- Mean present but no interval -> the mean and percent with no ±
- No mean -> en dash; the percent shows (–) when token_change_pct is None

## Terms

comparison cell, table cell, token delta, signed difference, interval display

## Connections

- **Location:** `bench/src/report.rs:218-229`
- **Signature:** `fn diff_cell(g: &Gate) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[report-signed-pct]]
- **Called by:** [[report-render]]
