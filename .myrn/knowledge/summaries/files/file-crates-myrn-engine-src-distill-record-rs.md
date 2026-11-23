---
id: file-crates-myrn-engine-src-distill-record-rs
type: summary
status: unverified
title: crates/myrn-engine/src/distill/record.rs
source: myrn summarize
about: file:crates/myrn-engine/src/distill/record.rs
anchors:
- file:crates/myrn-engine/src/distill/record.rs@68ea2dc551b7612c
---
# crates/myrn-engine/src/distill/record.rs

Engine methods that validate a submitted record, merge it with existing notes or dedupe it, write it as a note file, and sync the index, plus the session-end request that asks the agent to record what it learned. Main parts: record (checks, reference resolution, merge, write), the JSON and batch entry points, put_note and note_file for file paths, and the summary staleness checks.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[engine-record-json]], [[engine-record-any-json]], [[engine-record]], [[engine-put-note]], [[engine-note-file]], [[engine-current-session]], [[engine-records-request]], [[engine-summary-is-current]], [[engine-summary-outdated]], [[record-link]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-contradiction-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-signature-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-layout-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-paths-rs]], [[file-crates-myrn-store-src-sync-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-mod-rs]]
