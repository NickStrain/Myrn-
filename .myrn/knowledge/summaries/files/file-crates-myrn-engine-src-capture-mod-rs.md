---
id: file-crates-myrn-engine-src-capture-mod-rs
type: summary
status: unverified
title: crates/myrn-engine/src/capture/mod.rs
source: myrn summarize
about: file:crates/myrn-engine/src/capture/mod.rs
anchors:
- file:crates/myrn-engine/src/capture/mod.rs@857d592a3d9bb5f8
---
# crates/myrn-engine/src/capture/mod.rs

Session capture for the episode log: turns normalized events into redacted, repo-relative log entries, and drops paths outside the repo or matched by `.myrn/ignore`. Main parts are `to_entry` (event to entry), the text cleaners `clean` and `hide_ignored`, the path helpers `is_ignored`, `relative` and `normalize`, and unit tests. Submodules are consent, episode, outcome and redact.

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs`
- **In:** [[dir-crates-myrn-engine-src-capture]]
- **Defines:** [[capture-to-entry]], [[capture-clean]], [[capture-hide-ignored]], [[capture-is-ignored]], [[capture-relative]], [[capture-normalize]], [[tests-ignore]], [[tests-tool]]
- **Imports:** [[file-crates-myrn-engine-src-capture-consent-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-capture-outcome-rs]], [[file-crates-myrn-engine-src-capture-redact-rs]], [[file-crates-myrn-model-src-event-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-paths-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-lib-rs]]
