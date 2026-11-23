---
id: recall-first-paragraph
type: summary
status: unverified
title: recall::first_paragraph
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::first_paragraph
anchors:
- sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::first_paragraph@5a8f15b8be4a47d1
map_version: '2'
---
# recall::first_paragraph

Takes the text before the first blank line, collapses all whitespace runs to single spaces, and clips the result to max.

## Behaviour

- split is on '\n\n' only, so CRLF blank lines do not split
- 'a b\n c' -> 'a b c' before clipping
- empty or whitespace-only input -> empty string

## Terms

first paragraph, summary, blank line split, whitespace collapse, lead text, excerpt

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs:160-163`
- **Signature:** `fn first_paragraph(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-retrieve-recall-rs]]
- **Calls:** [[recall-clip]]
- **Called by:** [[engine-recall-text]]
