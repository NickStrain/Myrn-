---
id: review-score
type: summary
status: unverified
title: review::Score
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::Score
anchors:
- sym:crates/myrn-engine/src/review.rs#review::Score@1bff43f93057fff6
map_version: '2'
---
# review::Score

Tally of a marked checklist: counts of correct, vague, wrong, unmarked and ambiguous records, precision and not-wrong ratios, and per-type correct and marked counts.

## Behaviour

- precision = correct / (correct + vague + wrong), None when nothing is marked
- not_wrong = (correct + vague) / marked, so vague counts toward not wrong
- by_type lists every type seen, including types with zero marked records

## Terms

accuracy, hit rate, precision figure, score summary, tally, not-wrong ratio, per-type breakdown

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:255-268`
- **Signature:** `pub struct Score`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
