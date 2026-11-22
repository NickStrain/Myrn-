---
id: indexer-refresh
type: summary
status: unverified
title: indexer::refresh
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::refresh
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::refresh@4d015c2c2f851409
---
# indexer::refresh

Syncs the graph with the walked working tree and then with Git history, and returns an IndexReport. Files with unchanged size and mtime are skipped. Files whose mtime changed but whose hash matches only get size and mtime updated. Removed files are deleted, and changed files are parsed in parallel and persisted in one transaction. Edge resolution and module GC run only when something changed or the 'resolved' meta flag is missing. Git history errors are recorded in report.errors rather than returned. Writes the last_refresh meta key.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:53-153`
- **Signature:** `pub fn refresh( conn: &mut Connection, root: &Path, cfg: &Config, ignore_file: &Path, ) -> Result<IndexReport>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Calls:** [[git-index-history]], [[indexer-content-hash]], [[indexer-parse-all]], [[indexer-delete-file]], [[indexer-persist]], [[indexer-gc-modules]], [[symbols-resolve-all]], [[walk-walk]], [[graph-count]]
- **Called by:** [[engine-refresh]]
