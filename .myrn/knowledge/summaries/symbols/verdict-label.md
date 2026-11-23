---
id: verdict-label
type: summary
status: unverified
title: Verdict::label
source: myrn summarize
about: sym:bench/src/report.rs#report::Verdict::label
anchors:
- sym:bench/src/report.rs#report::Verdict::label@a915baf94a62df4e
map_version: '2'
---
# Verdict::label

Returns the upper-case display text for a verdict.

## Behaviour

- Pass -> 'PASS'
- Inconclusive -> 'INCONCLUSIVE', NoData -> 'NO DATA'
- Used in render headings and the Gate 1 and Step 20 lines

## Terms

display text, label, upper case, heading text, report wording

## Connections

- **Location:** `bench/src/report.rs:27-34`
- **Signature:** `fn label(self) -> &'static str`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-render]]
