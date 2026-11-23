---
id: file-crates-myrn-engine-src-summarize-rs
type: summary
status: unverified
title: crates/myrn-engine/src/summarize.rs
source: myrn summarize
about: file:crates/myrn-engine/src/summarize.rs
anchors:
- file:crates/myrn-engine/src/summarize.rs@510c6f848cdf4377
---
# crates/myrn-engine/src/summarize.rs

Builds the project map for 'myrn summarize'. Engine::summarize finds stale files, symbols and directories, asks the configured model command for summaries, and writes them as Obsidian summary notes under .myrn/knowledge/summaries/. The other items build each note's body and links, run model calls and parse their JSON answers, and run the jobs in parallel.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[summarize-options]], [[summarize-report]], [[summarize-entry]], [[summarize-existing]], [[engine-summarize]], [[engine-map-body]], [[summarize-filejob]], [[summarize-symout]], [[summarize-sym-out]], [[summarize-fileout]], [[summarize-dirjob]], [[summarize-summarize-file]], [[summarize-summarize-dir]], [[summarize-run-model]], [[summarize-ask-json]], [[summarize-parse-json]], [[summarize-parallel]], [[summarize-readme-excerpt]], [[summarize-title-of]], [[summarize-display-dir]], [[summarize-clean]], [[summarize-slug]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-sync-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-engine-src-viz-rs]], [[file-crates-myrn-engine-tests-summarize-rs]], [[file-crates-myrn-src-cmd-summarize-rs]]
