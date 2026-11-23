---
id: file-crates-myrn-store-src-sync-rs
type: summary
status: unverified
title: crates/myrn-store/src/sync.rs
source: myrn summarize
about: file:crates/myrn-store/src/sync.rs
anchors:
- file:crates/myrn-store/src/sync.rs@afe0645b20ab4d07
---
# crates/myrn-store/src/sync.rs

Keeps note files in step with the notes table and the code graph. The scan pass (`walk`, `sync`) stats note files and re-parses only changed ones; the link pass (`link` and its helpers) rebuilds note nodes, edges, anchors, summaries and issues from stored parses, and also derives record states and contradictions.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[sync-noteroot]], [[sync-notesreport]], [[sync-issue]], [[sync-notessummary]], [[sync-found]], [[sync-walk]], [[walk-rec]], [[sync-content-hash]], [[sync-body-hash]], [[sync-sync]], [[sync-row]], [[sync-resolver]], [[sync-segments]], [[sync-looks-like-sha]], [[resolver-note]], [[resolver-exists]], [[resolver-resolve-sync]], [[sync-add-edge]], [[sync-link]], [[sync-shared-contradictions]], [[sync-conflict]], [[sync-conflicts]], [[sync-set-local-verified]], [[sync-derive-states]], [[sync-drift]]
- **Imports:** [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-fts-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-claims-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-engine-src-validity-states-rs]], [[file-crates-myrn-engine-src-viz-rs]], [[file-crates-myrn-store-src-lib-rs]]
