---
id: pull-only-strip
type: summary
status: unverified
title: pull_only::strip
source: myrn summarize
about: sym:crates/myrn-adapters/src/pull_only.rs#pull_only::strip
anchors:
- sym:crates/myrn-adapters/src/pull_only.rs#pull_only::strip@554f402f2d7b2b28
---
# pull_only::strip

Returns the text with the first START...END span removed, along with one newline after END and any trailing newlines before START, joining the remaining parts with a blank line; returns the input unchanged if either marker is missing or END comes before START. Only the first block is removed, and whitespace other than newlines before START is not trimmed, so a whitespace-only prefix survives as non-empty.

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs:60-78`
- **Signature:** `fn strip(s: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-pull-only-rs]]
- **Called by:** [[pull-only-add]], [[pull-only-remove]]
