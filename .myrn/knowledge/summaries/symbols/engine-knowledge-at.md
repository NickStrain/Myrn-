---
id: engine-knowledge-at
type: summary
status: unverified
title: Engine::knowledge_at
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::knowledge_at
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::knowledge_at@792c4fc19b2f0dd2
map_version: '2'
---
# Engine::knowledge_at

Loads shared notes keyed by id, either from disk (`None`) or from the `.myrn/knowledge` tree at a revision or date, and returns a label with them.

## Behaviour

- revision with no knowledge directory -> empty map with the label, no error
- working tree: dot-prefixed names are skipped
- a note id that appears twice keeps the last one walked, since the map insert overwrites

## Terms

snapshot, shared notes, tree, working copy, notes at revision, point in time

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:356-394`
- **Signature:** `fn knowledge_at( &self, repo: &git2::Repository, at: Option<&str>, ) -> Result<(String, BTreeMap<String, Note>)>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[episode-summary]], [[knowledge-parse-note]], [[knowledge-walk-notes]], [[knowledge-resolve-point]], [[layout-knowledge-dir]]
- **Called by:** [[engine-diff-knowledge]]
