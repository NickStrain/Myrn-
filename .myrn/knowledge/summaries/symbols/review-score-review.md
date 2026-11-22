---
id: review-score-review
type: summary
status: unverified
title: review::score
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::score
anchors:
- sym:crates/myrn-engine/src/review.rs#review::score@2b10f54fc8bfffc4
---
# review::score

Parses a checklist back into a Score. A '## ' heading sets the current record type from its second ' · ' field, and each 'Mark:' line counts as correct, vague, or wrong when exactly one box is ticked with x or X, unmarked when none is, and ambiguous otherwise.

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:267-308`
- **Signature:** `pub fn score(text: &str) -> Score`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Called by:** [[real-main]], [[debug-review]]
- **Tests:** `marks_round_trip`
