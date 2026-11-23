---
id: engine-summarize
type: summary
status: unverified
title: Engine::summarize
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::Engine::summarize
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::Engine::summarize@e24f454a77fc0880
map_version: '2'
---
# Engine::summarize

Brings the project map up to date: loads earlier summary notes, marks stale files, symbols and directories, asks the model per stale file in parallel and per directory deepest-first, then writes changed notes and deletes notes for code that no longer exists.

## Behaviour

- dry_run true -> returns after counting: stale_dirs = directories without an existing note; no model call or write
- a symbol is stale unless its stored hash is a prefix of the current body hash AND map_version is '2'
- a symbol answer matches by exact qualified name, else by bare title only when that title is unique among the stale symbols sent for the file

## Terms

project map, stale detection, incremental update, directory rollup, orphan note cleanup, note sync, summary cache

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:107-505`
- **Signature:** `pub fn summarize( &mut self, opts: &Options, progress: &(dyn Fn(&str) + Sync), ) -> Result<Report>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[engine-refresh]], [[engine-note-roots]], [[seed-segments]], [[engine-map-body]], [[summarize-summarize-file]], [[summarize-summarize-dir]], [[summarize-parallel]], [[summarize-readme-excerpt]], [[summarize-title-of]], [[summarize-display-dir]], [[summarize-slug]], [[graph-nodes-where]], [[graph-neighbors]], [[note-extra-str]], [[note-set-extra]], [[note-summary-text]], [[note-summary-details]], [[frontmatter-render]], [[layout-file-name]], [[layout-knowledge-dir]], [[sync-sync]], [[resolver-exists]]
- **Called by:** [[summarize-run]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
