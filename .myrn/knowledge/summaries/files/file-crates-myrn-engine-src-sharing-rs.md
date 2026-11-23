---
id: file-crates-myrn-engine-src-sharing-rs
type: summary
status: unverified
title: crates/myrn-engine/src/sharing.rs
source: myrn summarize
about: file:crates/myrn-engine/src/sharing.rs
anchors:
- file:crates/myrn-engine/src/sharing.rs@77f6bab8238e56cb
---
# crates/myrn-engine/src/sharing.rs

Publishes a private record, and its private root cause, into the committed knowledge directory: publish_plan previews the exact committed text without writing, and publish writes, removes the private source, and syncs. Also holds the cleaning helpers (directive stripping, local path hiding) and the UTC date helpers.

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[sharing-publishplan]], [[sharing-publishitem]], [[engine-session-agent]], [[engine-publish-plan]], [[engine-publish]], [[engine-publisher]], [[sharing-clean]], [[sharing-local-paths]], [[sharing-today]], [[sharing-ymd]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-validity-states-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-directives-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-layout-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-sync-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-src-cmd-publish-rs]]
