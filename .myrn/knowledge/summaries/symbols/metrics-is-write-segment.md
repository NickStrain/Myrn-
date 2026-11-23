---
id: metrics-is-write-segment
type: summary
status: unverified
title: metrics::is_write_segment
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::is_write_segment
anchors:
- sym:bench/src/metrics.rs#metrics::is_write_segment@b013657b377a3861
map_version: '2'
---
# metrics::is_write_segment

Decides whether one shell segment writes a file: in-place sed, perl -i, tee, patch, git apply, or a redirect to a real file.

## Behaviour

- 'sed -n 1,20p a.py' -> false, and 'sed -i.bak' -> true
- 'perl -Mstrict -e ...' -> true, because any dash word containing 'i' counts
- 'git log -p' -> false, and any tee or patch word -> true

## Terms

write segment, in-place flag, patch tool, redirect write, file mutation

## Connections

- **Location:** `bench/src/metrics.rs:283-298`
- **Signature:** `fn is_write_segment(seg: &str) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-redirects-to-file]]
- **Called by:** [[metrics-is-exploration]]
