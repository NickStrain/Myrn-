---
id: metrics-is-write-segment
type: summary
status: unverified
title: metrics::is_write_segment
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::is_write_segment
anchors:
- sym:bench/src/metrics.rs#metrics::is_write_segment@b013657b377a3861
---
# metrics::is_write_segment

Classifies a single shell segment as a write by its first word (sed/gsed -i, perl with an i flag, tee, patch, git apply) or, by default, by a file redirect.

## Connections

- **Location:** `bench/src/metrics.rs:283-298`
- **Signature:** `fn is_write_segment(seg: &str) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-redirects-to-file]]
- **Called by:** [[metrics-is-exploration]]
