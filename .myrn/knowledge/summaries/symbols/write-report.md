---
id: write-report
type: summary
status: unverified
title: write_report
source: myrn summarize
about: sym:bench/src/main.rs#write_report
anchors:
- sym:bench/src/main.rs#write_report@5861e0f769c290ba
---
# write_report

Parses every non-empty line of results.jsonl as a RunRecord (error on any bad line), writes report.md via report::render with the title Gate 1: paired benchmark, and prints the gate verdict and reason from report::gate. It overwrites any existing report.md.

## Connections

- **Location:** `bench/src/main.rs:424-439`
- **Signature:** `fn write_report(dir: &Path) -> Result<()>`
- **In:** [[file-bench-src-main-rs]]
- **Calls:** [[report-gate-report]], [[report-render]]
- **Called by:** [[real-main]]
