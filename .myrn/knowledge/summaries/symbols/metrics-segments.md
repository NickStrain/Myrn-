---
id: metrics-segments
type: summary
status: unverified
title: metrics::segments
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::segments
anchors:
- sym:bench/src/metrics.rs#metrics::segments@470d35bc2de791fe
---
# metrics::segments

Splits a shell command on &, ;, |, and newlines, trims each part, and drops empty parts and `cd ` steps.

## Connections

- **Location:** `bench/src/metrics.rs:271-275`
- **Signature:** `fn segments(cmd: &str) -> impl Iterator<Item = &str>`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[metrics-is-exploration]], [[metrics-is-shell-write]]
