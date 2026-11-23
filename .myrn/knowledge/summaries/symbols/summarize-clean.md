---
id: summarize-clean
type: summary
status: unverified
title: summarize::clean
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::clean
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::clean@efe769cd94d87c4a
map_version: '2'
---
# summarize::clean

Collapses every run of whitespace, newlines included, into one space and trims the ends, so model text fits on one line.

## Behaviour

- input with newlines and runs of spaces -> single-spaced single line, trimmed
- only whitespace -> empty string
- newlines removed, so a '## ' heading can no longer start a line in the note

## Terms

whitespace normalize, one line, trim, collapse spaces, line break removal

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:928-930`
- **Signature:** `fn clean(s: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Called by:** [[summarize-sym-out]], [[summarize-summarize-dir]]
