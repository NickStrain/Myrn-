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

Session capture: turns normalized agent events into redacted episode log entries (the module doc says capture is off until consent). It holds the path rules (repo-relative only, no .git, gitignore filtering), text cleaning for prompts, commands and errors, and the unit tests for them.

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs`
- **In:** [[dir-crates-myrn-engine-src-capture]]
- **Defines:** [[capture-to-entry]], [[capture-clean]], [[capture-hide-ignored]], [[capture-is-ignored]], [[capture-relative]], [[capture-normalize]], [[tests-ignore]], [[tests-tool]]
- **Imports:** [[file-crates-myrn-engine-src-capture-consent-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-capture-outcome-rs]], [[file-crates-myrn-engine-src-capture-redact-rs]], [[file-crates-myrn-model-src-event-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-paths-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-lib-rs]]
