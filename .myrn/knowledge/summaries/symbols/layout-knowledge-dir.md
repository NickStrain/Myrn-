---
id: layout-knowledge-dir
type: summary
status: unverified
title: Layout::knowledge_dir
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::knowledge_dir
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::knowledge_dir@515471d60d476461
map_version: '2'
---
# Layout::knowledge_dir

Returns the shared knowledge directory inside the repo's `.myrn` directory.

## Behaviour

- -> <repo>/.myrn/knowledge
- contains one subdirectory per KNOWLEDGE_KINDS entry once create_dirs has run

## Terms

shared knowledge, knowledge base, repo knowledge, knowledge folder

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:62-64`
- **Signature:** `pub fn knowledge_dir(&self) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-init]], [[engine-note-roots]], [[engine-write-note]], [[engine-note-file]], [[engine-knowledge-at]], [[engine-publish-plan]], [[engine-summarize]], [[layout-create-dirs]]
- **Tests:** `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`, `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `map_is_written_linked_incremental_and_searchable`
