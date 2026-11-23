---
id: write-report
type: summary
status: unverified
title: write_report
source: myrn summarize
about: sym:bench/src/main.rs#write_report
anchors:
- sym:bench/src/main.rs#write_report@5861e0f769c290ba
map_version: '2'
---
# write_report

Reads results.jsonl from a run directory, writes report.md beside it, and prints the Gate 1 verdict and report path.

## Behaviour

- results.jsonl missing -> error naming the directory
- blank lines in results.jsonl skipped
- writes report.md, then prints the gate verdict and reason

## Terms

report generation, report.md, gate verdict, summary, render report, results summary

## Connections

- **Location:** `bench/src/main.rs:562-577`
- **Signature:** `fn write_report(dir: &Path) -> Result<()>`
- **In:** [[file-bench-src-main-rs]]
- **Calls:** [[report-gate-report]], [[report-render]]
- **Called by:** [[real-main]]
