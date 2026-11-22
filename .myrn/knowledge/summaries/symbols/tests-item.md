---
id: tests-item
type: summary
status: unverified
title: tests::item
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::tests::item
anchors:
- sym:crates/myrn-engine/src/review.rs#review::tests::item@5945aeb1c37e7410
---
# tests::item

Test helper that builds a minimal ReviewItem with the given id and fixed values: type 'fact', status 'unverified', title 't', body 'b', and empty evidence, links, code, and origin.

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:314-327`
- **Signature:** `fn item(id: &str) -> ReviewItem`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Tests:** `sampling_is_reproducible`, `marks_round_trip`
