---
id: directives-sentences
type: summary
status: unverified
title: directives::sentences
source: myrn summarize
about: sym:crates/myrn-store/src/notes/directives.rs#notes::directives::sentences
anchors:
- sym:crates/myrn-store/src/notes/directives.rs#notes::directives::sentences@f0f4afaf1b74ddaf
map_version: '2'
---
# directives::sentences

Splits one line after each period, exclamation mark, or question mark that is followed by ASCII whitespace. Each piece keeps its trailing whitespace, and the split uses byte positions so it is safe for multi-byte text.

## Behaviour

- 'Use it. Then stop' -> ['Use it. ', 'Then stop'].
- 'v1.2 is out' -> one piece, because a dot needs whitespace directly after it to split.
- Empty line -> empty list; text after the last terminator becomes the final piece unless it is empty.

## Terms

sentence split, sentence boundary, split on punctuation, segment text, sentence segmentation

## Connections

- **Location:** `crates/myrn-store/src/notes/directives.rs:60-84`
- **Signature:** `fn sentences(line: &str) -> Vec<&str>`
- **In:** [[file-crates-myrn-store-src-notes-directives-rs]]
- **Called by:** [[directives-strip]]
