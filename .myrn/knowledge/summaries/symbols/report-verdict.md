---
id: report-verdict
type: summary
status: unverified
title: report::Verdict
source: myrn summarize
about: sym:bench/src/report.rs#report::Verdict
anchors:
- sym:bench/src/report.rs#report::Verdict@0f8d28207de96e94
---
# report::Verdict

Gate outcome enum (Pass, Inconclusive, Fail, NoData) serialized as snake_case. Inconclusive means tokens fell on average but the 95% interval includes zero.

## Connections

- **Location:** `bench/src/report.rs:18-24`
- **Signature:** `pub enum Verdict`
- **In:** [[file-bench-src-report-rs]]
