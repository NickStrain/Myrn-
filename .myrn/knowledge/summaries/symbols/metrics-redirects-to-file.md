---
id: metrics-redirects-to-file
type: summary
status: unverified
title: metrics::redirects_to_file
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::redirects_to_file
anchors:
- sym:bench/src/metrics.rs#metrics::redirects_to_file@807316dfe01458c4
map_version: '2'
---
# metrics::redirects_to_file

Returns true if the segment has a > or >> redirect whose target is a real file and not a descriptor duplicate or /dev path.

## Behaviour

- 'echo x >> notes.txt' -> true
- 'grep -rn foo . > /dev/null' and 'cargo 2>&1' -> false
- 'cmd 2> err.log' -> false, because a digit before > is treated as a descriptor redirect

## Terms

output redirect, stdout redirection, file redirect, append, descriptor duplication

## Connections

- **Location:** `bench/src/metrics.rs:301-322`
- **Signature:** `fn redirects_to_file(seg: &str) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[metrics-is-write-segment]]
