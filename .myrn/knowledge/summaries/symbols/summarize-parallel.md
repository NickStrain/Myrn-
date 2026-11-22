---
id: summarize-parallel
type: summary
status: unverified
title: summarize::parallel
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::parallel
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::parallel@5b8116ab343dde1b
---
# summarize::parallel

Maps f over items on scoped threads with at most jobs running at once (clamped to 1 through 16), returning results in input order.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:779-797`
- **Signature:** `fn parallel<T: Sync, R: Send>(items: &[T], jobs: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Called by:** [[engine-summarize]]
- **Tests:** `parallel_keeps_order`
