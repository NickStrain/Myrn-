---
id: engine-diff-knowledge
type: summary
status: unverified
title: Engine::diff_knowledge
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::diff_knowledge
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::diff_knowledge@f00486991820cb11
map_version: '2'
---
# Engine::diff_knowledge

Compares shared knowledge at `from` and `to` (a revision or date; `None` means the working tree) and sorts records into added, removed and changed.

## Behaviour

- unresolvable from or to -> error from resolve_point
- private notes are never compared
- a map summary that is added, removed or changed only increments map_changes

## Terms

diff, compare, between two points, knowledge changes, what changed, revision range

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:309-353`
- **Signature:** `pub fn diff_knowledge(&self, from: &str, to: Option<&str>) -> Result<KnowledgeDiff>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[engine-root]], [[engine-context]], [[engine-knowledge-at]], [[tests-item]], [[note-display-title]]
- **Called by:** [[diff-run]]
- **Tests:** `diff_compares_shared_knowledge_between_points`
