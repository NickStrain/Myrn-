---
id: tests-item
type: summary
status: unverified
title: tests::item
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::tests::item
anchors:
- sym:crates/myrn-engine/src/review.rs#review::tests::item@5945aeb1c37e7410
map_version: '2'
---
# tests::item

Test helper that builds a minimal ReviewItem with the given id, type 'fact', status 'unverified', title 't', body 'b', and no evidence, links, code or origin.

## Behaviour

- item('f3') -> id 'f3' with every other field fixed
- used to build sample lists for the sampling and round-trip tests

## Terms

test fixture, dummy record, stub item, test helper, minimal record, sample item

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:318-331`
- **Signature:** `fn item(id: &str) -> ReviewItem`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Called by:** [[engine-diff-knowledge]]
- **Tests:** `sampling_is_reproducible`, `marks_round_trip`
