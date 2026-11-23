---
id: debug-render-score
type: summary
status: unverified
title: debug::render_score
source: myrn summarize
about: sym:crates/myrn/src/cmd/debug.rs#cmd::debug::render_score
anchors:
- sym:crates/myrn/src/cmd/debug.rs#cmd::debug::render_score@840d21f83a816982
map_version: '2'
---
# debug::render_score

Formats a review::Score as a plain-text report with a precision line, a marks line, and one indented row per type. Percentages are rounded, and missing values show as a dash.

## Behaviour

- precision Some(0.8333) -> "83%"; precision None -> "–"
- marked total = correct + vague + wrong, so unmarked records are not counted in the precision denominator
- by_type entries (t, c, m) -> " {t:<11} c of m correct", with the type name left-padded to 11 characters

## Terms

score report, precision, marks summary, per-type breakdown, percent formatting, score text, summary text

## Connections

- **Location:** `crates/myrn/src/cmd/debug.rs:51-73`
- **Signature:** `pub fn render_score(s: &review::Score) -> String`
- **In:** [[file-crates-myrn-src-cmd-debug-rs]]
- **Called by:** [[debug-review]]
