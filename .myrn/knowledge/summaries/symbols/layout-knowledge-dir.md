---
id: layout-knowledge-dir
type: summary
status: unverified
title: Layout::knowledge_dir
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::knowledge_dir
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::knowledge_dir@515471d60d476461
---
# Layout::knowledge_dir

Returns repo_dir/knowledge, the shared knowledge tree inside the repo.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:62-64`
- **Signature:** `pub fn knowledge_dir(&self) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-init]], [[engine-note-roots]], [[engine-write-note]], [[engine-note-file]], [[engine-summarize]], [[layout-create-dirs]]
- **Tests:** `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `map_is_written_linked_incremental_and_searchable`
