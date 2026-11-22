---
id: review-checklist
type: summary
status: unverified
title: review::checklist
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::checklist
anchors:
- sym:crates/myrn-engine/src/review.rs#review::checklist@00fb588bebadd18b
---
# review::checklist

Renders items as a Markdown checklist with a title, a legend for the three marks, and per item a numbered heading, the fixed MARK_LINE, origin and source lines, the quoted body, evidence, links, and a fenced code block for each code target marked '(changed since recorded)' when stale. Items with no code get 'Code: none linked'.

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:188-248`
- **Signature:** `pub fn checklist(items: &[ReviewItem], title: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Called by:** [[real-main]], [[debug-review]]
- **Tests:** `marks_round_trip`
