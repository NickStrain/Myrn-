---
id: review-sample
type: summary
status: unverified
title: review::sample
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::sample
anchors:
- sym:crates/myrn-engine/src/review.rs#review::sample@362faab9cfad7f8d
---
# review::sample

Deterministically shuffles items with Fisher-Yates driven by an xorshift generator seeded from seed (0 is treated as 1), then keeps the first n. If n is 0 or at least the item count, it returns all items unshuffled.

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:168-183`
- **Signature:** `pub fn sample(mut items: Vec<ReviewItem>, n: usize, seed: u64) -> Vec<ReviewItem>`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Called by:** [[real-main]], [[debug-review]]
- **Tests:** `sampling_is_reproducible`
