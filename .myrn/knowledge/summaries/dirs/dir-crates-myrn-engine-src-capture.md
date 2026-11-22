---
id: dir-crates-myrn-engine-src-capture
type: summary
status: unverified
title: crates/myrn-engine/src/capture/
source: myrn summarize
about: module:crates/myrn-engine/src/capture
anchors:
- module:crates/myrn-engine/src/capture@58d3b988dc2f45bf
---
# crates/myrn-engine/src/capture/

Session capture for the engine: converts normalized agent events into redacted episode log entries, gated by consent (consent.rs, checked via may_record; capture is off until granted). mod.rs holds path rules (repo-relative only, skips .git and gitignored files) and text cleaning; redact.rs scrubs secrets before any write; outcome.rs detects test runs and recent commits; episode.rs folds the log into an Episode and persists it as a graph node with touches edges.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-capture-consent-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-capture-outcome-rs]], [[file-crates-myrn-engine-src-capture-redact-rs]]
