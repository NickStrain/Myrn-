---
id: review-checklist
type: summary
status: unverified
title: review::checklist
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::checklist
anchors:
- sym:crates/myrn-engine/src/review.rs#review::checklist@00fb588bebadd18b
map_version: '2'
---
# review::checklist

Renders items as Markdown: a title, marking instructions, then per item a heading, the Mark line, the quoted body, evidence, links and code blocks, with changed code flagged.

## Behaviour

- empty list -> only the title and instructions, with a count of 0
- item with no code -> a 'Code: none linked' line
- changed code -> its location gets the suffix '(changed since recorded)'

## Terms

markdown checklist, review sheet, render, mark line, quoted body, code fence, flagged stale code, checkbox list

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:192-252`
- **Signature:** `pub fn checklist(items: &[ReviewItem], title: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Called by:** [[real-main]], [[debug-review]]
- **Tests:** `marks_round_trip`
