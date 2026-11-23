---
id: dir-crates-myrn-engine-src-capture
type: summary
status: unverified
title: crates/myrn-engine/src/capture/
source: myrn summarize
about: module:crates/myrn-engine/src/capture
anchors:
- module:crates/myrn-engine/src/capture@8cc5450ba3ff599c
---
# crates/myrn-engine/src/capture/

Session capture for the episode log: mod.rs turns normalized events into redacted, repo-relative log entries via to_entry, dropping paths outside the repo or matched by .myrn/ignore. consent.rs gates recording through may_record, which reads the one-time consent flag and agent mode from the private state.json, and redact.rs scrubs secrets before anything is written. episode.rs folds log entries into an Episode and persists it as a graph node with touches edges, while outcome.rs supplies is_test_command and commits_since for attaching outcomes.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-capture-consent-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-capture-outcome-rs]], [[file-crates-myrn-engine-src-capture-redact-rs]]
