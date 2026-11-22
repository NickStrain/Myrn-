---
id: file-crates-myrn-engine-tests-summarize-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/summarize.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/summarize.rs
anchors:
- file:crates/myrn-engine/tests/summarize.rs@7d0fff00285986e4
---
# crates/myrn-engine/tests/summarize.rs

Integration test for the summarize pipeline. It checks that summaries are written as Obsidian notes linked by [[id]], that unchanged code makes no model calls, that only changed files are re-asked, and that notes for removed code are deleted. It runs against a temp git repo and a stub model command.

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write-summarize]], [[repo-summarize]], [[model]], [[calls]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-summarize-rs]]
