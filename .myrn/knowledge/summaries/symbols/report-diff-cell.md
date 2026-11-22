---
id: report-diff-cell
type: summary
status: unverified
title: report::diff_cell
source: myrn summarize
about: sym:bench/src/report.rs#report::diff_cell
anchors:
- sym:bench/src/report.rs#report::diff_cell@773ae734725e14bf
---
# report::diff_cell

Renders a Gate's token diff as '+X.Xk ± Y.Yk (signed pct)', dropping the CI or percent parts when absent and rendering '–' when there is no mean.

## Connections

- **Location:** `bench/src/report.rs:218-229`
- **Signature:** `fn diff_cell(g: &Gate) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[report-signed-pct]]
- **Called by:** [[report-render]]
