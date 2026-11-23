---
id: summarize-parallel
type: summary
status: unverified
title: summarize::parallel
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::parallel
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::parallel@5b8116ab343dde1b
map_version: '2'
---
# summarize::parallel

Runs f over the items on up to jobs threads (kept between 1 and 16 and never more than the item count) and returns results in input order.

## Behaviour

- jobs 0 -> one thread; jobs 40 -> 16 threads
- empty items -> returns an empty Vec
- results keep input order even when calls finish out of order

## Terms

thread pool, concurrency, ordered results, worker threads, bounded parallelism

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:866-884`
- **Signature:** `fn parallel<T: Sync, R: Send>(items: &[T], jobs: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Called by:** [[engine-summarize]]
- **Tests:** `parallel_keeps_order`
