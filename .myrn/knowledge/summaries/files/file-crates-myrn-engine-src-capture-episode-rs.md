---
id: file-crates-myrn-engine-src-capture-episode-rs
type: summary
status: unverified
title: crates/myrn-engine/src/capture/episode.rs
source: myrn summarize
about: file:crates/myrn-engine/src/capture/episode.rs
anchors:
- file:crates/myrn-engine/src/capture/episode.rs@ab0e1ffe555f5a7c
---
# crates/myrn-engine/src/capture/episode.rs

Turns a session's event log into an Episode (task, files read and edited, symbols, commands, errors, test results, commits, outcome) and writes it to the graph as an episode node with touches edges. Holds the Episode struct, its outcome and summary methods, fold, persist, clip, and a unit test for fold.

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs`
- **In:** [[dir-crates-myrn-engine-src-capture]]
- **Defines:** [[episode-episode]], [[episode-outcome]], [[episode-summary]], [[episode-fold]], [[episode-persist]], [[episode-clip]], [[tests-e]]
- **Imports:** [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-distill-mechanical-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-src-cmd-log-rs]]
