---
id: pull-only-strip
type: summary
status: unverified
title: pull_only::strip
source: myrn summarize
about: sym:crates/myrn-adapters/src/pull_only.rs#pull_only::strip
anchors:
- sym:crates/myrn-adapters/src/pull_only.rs#pull_only::strip@554f402f2d7b2b28
map_version: '2'
---
# pull_only::strip

Private helper that returns the text with the first marker block and one newline after its END marker removed. It trims trailing newlines from the text before the block so no extra blank lines are left.

## Behaviour

- block at end with text before -> text before, then one newline
- block in the middle -> text before, blank line, text after
- END before START, or either marker missing -> input returned unchanged

## Terms

remove block, trim block, blank line, marker removal, text without block, strip markers

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs:60-78`
- **Signature:** `fn strip(s: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-pull-only-rs]]
- **Called by:** [[pull-only-add]], [[pull-only-remove]]
