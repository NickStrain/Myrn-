---
id: review-score-review
type: summary
status: unverified
title: review::score
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::score
anchors:
- sym:crates/myrn-engine/src/review.rs#review::score@2b10f54fc8bfffc4
map_version: '2'
---
# review::score

Parses the Mark lines of a checklist and tallies each record by its single ticked box, taking the record type from its '## ' heading.

## Behaviour

- exactly one [x] or [X] on correct, vague or wrong -> that count
- no box ticked -> unmarked; two or more ticked -> ambiguous
- heading '## 3. f1 · fact · unverified' -> type is 'fact' (second field split on ' · ')

## Terms

parse marks, read back, tick box, count marks, tally, ambiguous marks, heading type, checkbox parsing

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:271-312`
- **Signature:** `pub fn score(text: &str) -> Score`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Called by:** [[real-main]], [[debug-review]]
- **Tests:** `marks_round_trip`
