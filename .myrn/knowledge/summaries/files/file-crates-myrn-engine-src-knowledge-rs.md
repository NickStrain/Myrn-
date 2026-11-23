---
id: file-crates-myrn-engine-src-knowledge-rs
type: summary
status: unverified
title: crates/myrn-engine/src/knowledge.rs
source: myrn summarize
about: file:crates/myrn-engine/src/knowledge.rs
anchors:
- file:crates/myrn-engine/src/knowledge.rs@c5f81fb0585deb9e
---
# crates/myrn-engine/src/knowledge.rs

Read-side views over learned knowledge: the record list used by `log` (`Engine::records`), provenance for one record (`Engine::trace`), changes in shared knowledge between two revisions or dates (`Engine::diff_knowledge`), and permanent removal (`Engine::forget_plan` and `Engine::forget`). Helpers parse note files, read notes from disk or Git trees, resolve revisions or YYYY-MM-DD dates to commits, and list a file's commit history.

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[knowledge-recordrow]], [[knowledge-recordfilter]], [[knowledge-trace]], [[knowledge-tracedcommit]], [[knowledge-tracedanchor]], [[knowledge-knowledgediff]], [[knowledge-diffitem]], [[knowledge-forgotten]], [[engine-records]], [[engine-record-row]], [[engine-trace]], [[engine-diff-knowledge]], [[engine-knowledge-at]], [[engine-forget-plan]], [[engine-forget]], [[knowledge-parse-note]], [[knowledge-walk-notes]], [[knowledge-resolve-point]], [[knowledge-date-end]], [[knowledge-file-history]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-paths-rs]], [[file-crates-myrn-store-src-sync-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-engine-src-viz-rs]], [[file-crates-myrn-engine-tests-history-rs]], [[file-crates-myrn-src-cmd-log-rs]]
