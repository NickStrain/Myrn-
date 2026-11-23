---
id: metrics-segments
type: summary
status: unverified
title: metrics::segments
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::segments
anchors:
- sym:bench/src/metrics.rs#metrics::segments@470d35bc2de791fe
map_version: '2'
---
# metrics::segments

Splits a shell command line on &, ;, | and newline, trims each part, and drops empty parts and cd steps.

## Behaviour

- 'cd x && sed -i ...' -> only the sed segment
- Splitting on & also breaks redirects: '2>&1' becomes '2>' and '1'
- Quoted separators are split too, since quoting is not parsed

## Terms

command splitting, pipeline parts, shell chain, command segments, cd skipping

## Connections

- **Location:** `bench/src/metrics.rs:271-275`
- **Signature:** `fn segments(cmd: &str) -> impl Iterator<Item = &str>`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[metrics-is-exploration]], [[metrics-is-shell-write]]
