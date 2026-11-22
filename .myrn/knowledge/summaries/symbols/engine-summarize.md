---
id: engine-summarize
type: summary
status: unverified
title: Engine::summarize
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::Engine::summarize
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::Engine::summarize@b5629a4bfc1da210
---
# Engine::summarize

Refreshes the store, diffs node body hashes against existing summary notes to find stale files, symbols and directories, asks the model for each (files first, then directories deepest-first), and writes or removes summary notes, then syncs the vault. Takes `Options` (dry_run, limit, jobs, command) and a progress callback, and returns a `Report`; with dry_run or limit it only reports or defers work, and it writes no notes in dry-run mode.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:94-480`
- **Signature:** `pub fn summarize( &mut self, opts: &Options, progress: &(dyn Fn(&str) + Sync), ) -> Result<Report>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[engine-refresh]], [[engine-note-roots]], [[seed-segments]], [[engine-map-body]], [[summarize-summarize-file]], [[summarize-summarize-dir]], [[summarize-parallel]], [[summarize-readme-excerpt]], [[summarize-title-of]], [[summarize-display-dir]], [[summarize-slug]], [[graph-nodes-where]], [[graph-neighbors]], [[note-summary-text]], [[frontmatter-render]], [[layout-file-name]], [[layout-knowledge-dir]], [[sync-sync]], [[resolver-exists]]
- **Called by:** [[summarize-run]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
