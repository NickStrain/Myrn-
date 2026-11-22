---
id: metrics-is-exploration
type: summary
status: unverified
title: metrics::is_exploration
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::is_exploration
anchors:
- sym:bench/src/metrics.rs#metrics::is_exploration@706bd8a077952502
---
# metrics::is_exploration

Returns true for read/search tools, myrn MCP tools, and Bash commands where a segment is a look-only command (cat, grep, git log, etc.) and not a write segment.

## Connections

- **Location:** `bench/src/metrics.rs:254-268`
- **Signature:** `fn is_exploration(name: &str, input: &Value) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-segments]], [[metrics-is-write-segment]]
- **Called by:** [[metrics-parse]]
