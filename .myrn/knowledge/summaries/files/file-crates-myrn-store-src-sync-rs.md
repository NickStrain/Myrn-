---
id: file-crates-myrn-store-src-sync-rs
type: summary
status: unverified
title: crates/myrn-store/src/sync.rs
source: myrn summarize
about: file:crates/myrn-store/src/sync.rs
anchors:
- file:crates/myrn-store/src/sync.rs@bf2e48dd52d1e19c
---
# crates/myrn-store/src/sync.rs

Keeps the notes table and note-derived graph rows in step with note files on disk. A stat-based scan re-parses only changed files, then a link pass rebuilds nodes, edges, anchors, summaries and issues from the stored parses. Main parts are the scan (walk, sync), the link resolver (Resolver, link) and reporting (summary, issues).

## Connections

- **Location:** `crates/myrn-store/src/sync.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[sync-noteroot]], [[sync-notesreport]], [[sync-issue]], [[sync-notessummary]], [[sync-found]], [[sync-walk]], [[walk-rec]], [[sync-content-hash]], [[sync-sync]], [[sync-row]], [[sync-resolver]], [[sync-segments]], [[sync-looks-like-sha]], [[resolver-note]], [[resolver-exists]], [[resolver-resolve-sync]], [[sync-add-edge]], [[sync-link]], [[sync-summary]], [[sync-issues]]
- **Imports:** [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-fts-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-store-src-lib-rs]]
