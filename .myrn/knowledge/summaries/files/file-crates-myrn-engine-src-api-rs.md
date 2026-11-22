---
id: file-crates-myrn-engine-src-api-rs
type: summary
status: unverified
title: crates/myrn-engine/src/api.rs
source: myrn summarize
about: file:crates/myrn-engine/src/api.rs
anchors:
- file:crates/myrn-engine/src/api.rs@dccb1b75749f8055
---
# crates/myrn-engine/src/api.rs

Public engine API that adapters, MCP tools and the CLI call; it owns the Engine struct (layout, config, project id, SQLite Db) and exposes init/open/refresh/rebuild, lookup and query methods (explain, impact, history, context, brief, status, doctor), and the episode capture pipeline that records hook events into per-session logs. Nothing else should touch the stores directly.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[api-engine]], [[api-lookup]], [[lookup-render]], [[api-explained]], [[explained-render]], [[api-pack]], [[api-historyentry]], [[api-status]], [[api-level]], [[api-check]], [[api-check-api]], [[api-located]], [[api-locate]], [[engine-init]], [[engine-open]], [[engine-open-located]], [[engine-root]], [[engine-db]], [[engine-refresh]], [[engine-note-roots]], [[engine-write-note]], [[engine-note-issues]], [[engine-refresh-if-older]], [[engine-rebuild]], [[engine-find]]
- **Imports:** [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-config-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-lang-mod-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-pack-budget-rs]], [[file-crates-myrn-engine-src-pack-cards-rs]], [[file-crates-myrn-engine-src-pack-log-rs]], [[file-crates-myrn-engine-src-pack-render-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]], [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-engine-src-retrieve-walks-rs]], [[file-crates-myrn-model-src-event-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-paths-rs]], [[file-crates-myrn-store-src-sync-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]], [[file-crates-myrn-engine-src-review-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-engine-tests-gate0-rs]], [[file-crates-myrn-engine-tests-notes-rs]], [[file-crates-myrn-engine-tests-packing-rs]], [[file-crates-myrn-engine-tests-records-rs]], [[file-crates-myrn-engine-tests-summarize-rs]], [[file-crates-myrn-mcp-src-server-rs]], [[file-crates-myrn-mcp-src-tools-context-rs]], [[file-crates-myrn-mcp-src-tools-explain-rs]], [[file-crates-myrn-mcp-src-tools-history-rs]], [[file-crates-myrn-mcp-src-tools-impact-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]], [[file-crates-myrn-mcp-src-tools-recall-rs]], [[file-crates-myrn-mcp-src-tools-record-rs]], [[file-crates-myrn-mcp-tests-protocol-rs]], [[file-crates-myrn-src-cmd-doctor-rs]], [[file-crates-myrn-src-cmd-hook-rs]], [[file-crates-myrn-src-cmd-init-rs]], [[file-crates-myrn-src-cmd-mod-rs]], [[file-crates-myrn-src-cmd-rebuild-rs]]
