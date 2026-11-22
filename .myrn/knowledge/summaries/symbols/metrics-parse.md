---
id: metrics-parse
type: summary
status: unverified
title: metrics::parse
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::parse
anchors:
- sym:bench/src/metrics.rs#metrics::parse@b11045000264239c
---
# metrics::parse

Parses newline-delimited stream-json into a Session; skips unparseable lines, makes absolute paths root-relative, counts exploration before the first edit to a path in `touches` (or any edit if `touches` is empty), and after a Stop-hook record request switches to counting only the recording phase. Gotcha: usage is de-duplicated per message id by taking the max, and the answer is the last text before the record request.

## Connections

- **Location:** `bench/src/metrics.rs:100-252`
- **Signature:** `pub fn parse(stream: &str, root: &str, touches: &[String]) -> Session`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-is-exploration]]
- **Called by:** [[rescore-runs]], [[runner-run-inner]]
- **Tests:** `parses_a_transcript`, `shell_reads_and_edits_count`, `recording_phase_is_split_off`
