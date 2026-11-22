---
id: metrics-redirects-to-file
type: summary
status: unverified
title: metrics::redirects_to_file
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::redirects_to_file
anchors:
- sym:bench/src/metrics.rs#metrics::redirects_to_file@807316dfe01458c4
---
# metrics::redirects_to_file

Scans for > or >> whose target is a real file, ignoring fd duplications like 2>&1 and targets under /dev/.

## Connections

- **Location:** `bench/src/metrics.rs:301-322`
- **Signature:** `fn redirects_to_file(seg: &str) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[metrics-is-write-segment]]
