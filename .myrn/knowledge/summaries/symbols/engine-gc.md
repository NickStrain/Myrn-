---
id: engine-gc
type: summary
status: unverified
title: Engine::gc
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::Engine::gc
anchors:
- sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::Engine::gc@374ab28fbb275a07
map_version: '2'
---
# Engine::gc

Refreshes the store, removes duplicate note copies, drops index rows pointing at missing targets, and compacts session logs whose newest entry is older than older_than_days. With dry_run it only counts.

## Behaviour

- dry_run true -> counts filled in; no file removed, no row deleted, no log rewritten
- duplicate removed only if body words, links and anchors match the indexed copy and the key differs; else listed in duplicates_differing
- non-dry run: removed duplicates trigger a note sync, compacted logs trigger episode reindex, then WAL checkpoint and VACUUM

## Terms

garbage collection, tidy, prune, dedupe notes, dangling index rows, orphan rows, session log compaction, vacuum

## Connections

- **Location:** `crates/myrn-engine/src/validity/gc.rs:34-138`
- **Signature:** `pub fn gc(&mut self, older_than_days: u64, dry_run: bool) -> Result<GcReport>`
- **In:** [[file-crates-myrn-engine-src-validity-gc-rs]]
- **Calls:** [[engine-refresh]], [[engine-note-roots]], [[engine-reindex-episodes]], [[dedupe-find-id]], [[engine-note-file]], [[gc-compact]], [[gc-now]], [[episodes-read]], [[episodes-list]], [[frontmatter-parse]], [[layout-episodes-dir]], [[sync-sync]]
- **Called by:** [[gc-run]]
- **Tests:** `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`
