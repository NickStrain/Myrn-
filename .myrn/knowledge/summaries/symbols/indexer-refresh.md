---
id: indexer-refresh
type: summary
status: unverified
title: indexer::refresh
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::refresh
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::refresh@4d015c2c2f851409
map_version: '2'
---
# indexer::refresh

Walks root and skips files whose size and mtime match the files table, then reads and hashes the rest and, in one transaction, deletes removed files and re-parses changed ones. If anything was parsed or removed, or the 'resolved' meta flag is missing, it runs gc_modules and resolve_all, then indexes Git history and writes last_refresh.

## Behaviour

- same hash but new mtime -> only mtime and size updated, no re-parse
- unreadable file -> skipped, existing row left unchanged
- parse error -> persisted with error text and added to errors; Git failure -> added to errors, call still succeeds

## Terms

incremental, sync, update index, reindex, diff, stale files, walk, rebuild

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:53-153`
- **Signature:** `pub fn refresh( conn: &mut Connection, root: &Path, cfg: &Config, ignore_file: &Path, ) -> Result<IndexReport>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Calls:** [[git-index-history]], [[indexer-content-hash]], [[indexer-parse-all]], [[indexer-delete-file]], [[indexer-persist]], [[indexer-gc-modules]], [[symbols-resolve-all]], [[walk-walk]], [[graph-count]]
- **Called by:** [[engine-refresh]]
