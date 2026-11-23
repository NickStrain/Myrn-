---
id: review-sample
type: summary
status: unverified
title: review::sample
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::sample
anchors:
- sym:crates/myrn-engine/src/review.rs#review::sample@362faab9cfad7f8d
map_version: '2'
---
# review::sample

Returns n items chosen by a seeded Fisher-Yates shuffle driven by a xorshift generator, so the same seed and input always give the same result.

## Behaviour

- n is 0 or at least the item count -> all items returned in original order, unshuffled
- seed 0 is treated as 1
- same seed gives the same subset; a different seed gives a different subset

## Terms

random subset, shuffle, reproducible, seeded random, xorshift, deterministic pick, truncate, subset

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:172-187`
- **Signature:** `pub fn sample(mut items: Vec<ReviewItem>, n: usize, seed: u64) -> Vec<ReviewItem>`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Called by:** [[real-main]], [[debug-review]]
- **Tests:** `sampling_is_reproducible`
