---
id: metrics-is-exploration
type: summary
status: unverified
title: metrics::is_exploration
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::is_exploration
anchors:
- sym:bench/src/metrics.rs#metrics::is_exploration@706bd8a077952502
map_version: '2'
---
# metrics::is_exploration

Decides whether one tool call only reads or searches, counting Read, Grep, Glob, LS, NotebookRead, myrn tools, and Bash commands whose segments are read-only look commands.

## Behaviour

- Edit or Write -> false
- Bash 'sed -i ...' -> false, since the segment is a write
- Bash 'grep x && sed -i ...' -> true, because the grep segment counts as a look

## Terms

reconnaissance, read-only tool, search call, look command, navigation

## Connections

- **Location:** `bench/src/metrics.rs:254-268`
- **Signature:** `fn is_exploration(name: &str, input: &Value) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-segments]], [[metrics-is-write-segment]]
- **Called by:** [[metrics-parse]]
