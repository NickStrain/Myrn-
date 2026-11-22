---
id: review-score
type: summary
status: unverified
title: review::Score
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::Score
anchors:
- sym:crates/myrn-engine/src/review.rs#review::Score@1bff43f93057fff6
---
# review::Score

Tallies of correct, vague, wrong, unmarked, and ambiguous (more than one box ticked) marks, plus precision (correct / marked) and not_wrong ((correct + vague) / marked), both None when nothing is marked, and per-type (type, correct, marked) counts. Ambiguous and unmarked items are excluded from marked and from by_type.

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:251-264`
- **Signature:** `pub struct Score`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
