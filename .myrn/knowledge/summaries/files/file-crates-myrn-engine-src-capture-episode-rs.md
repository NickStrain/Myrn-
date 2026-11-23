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

Builds an Episode, a per-session record of prompts, files read and edited, symbols touched, commands, errors, test runs, commits and outcome. fold replays a session's log entries into an Episode, and persist writes it as an episode graph node with touches edges. The file also has the clip string helper and one test.

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs`
- **In:** [[dir-crates-myrn-engine-src-capture]]
- **Defines:** [[episode-episode]], [[episode-outcome]], [[episode-summary]], [[episode-fold]], [[episode-persist]], [[episode-clip]], [[tests-e]]
- **Imports:** [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-distill-mechanical-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-src-cmd-log-rs]], [[file-crates-myrn-src-cmd-trace-rs]]
