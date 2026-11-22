---
id: recall-first-paragraph
type: summary
status: unverified
title: recall::first_paragraph
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::first_paragraph
anchors:
- sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::first_paragraph@5a8f15b8be4a47d1
---
# recall::first_paragraph

Takes the first blank-line-separated paragraph, collapses its whitespace, and clips it to max characters.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs:157-160`
- **Signature:** `fn first_paragraph(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-retrieve-recall-rs]]
- **Calls:** [[recall-clip]]
- **Called by:** [[engine-recall-text]]
