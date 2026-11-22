---
id: debug-render-score
type: summary
status: unverified
title: debug::render_score
source: myrn summarize
about: sym:crates/myrn/src/cmd/debug.rs#cmd::debug::render_score
anchors:
- sym:crates/myrn/src/cmd/debug.rs#cmd::debug::render_score@840d21f83a816982
---
# debug::render_score

Formats a Score as a plain-text report: a precision line (correct of marked, with precision and not-wrong percentages, where a missing value shows a dash), a marks line (correct, vague, wrong, unmarked, and multi-box counts), then one indented line per by_type tuple giving type, correct, and marked.

## Connections

- **Location:** `crates/myrn/src/cmd/debug.rs:51-73`
- **Signature:** `pub fn render_score(s: &review::Score) -> String`
- **In:** [[file-crates-myrn-src-cmd-debug-rs]]
- **Called by:** [[debug-review]]
