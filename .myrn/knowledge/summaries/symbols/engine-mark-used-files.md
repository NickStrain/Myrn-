---
id: engine-mark-used-files
type: summary
status: unverified
title: Engine::mark_used_files
source: myrn summarize
about: sym:crates/myrn-engine/src/learn.rs#learn::Engine::mark_used_files
anchors:
- sym:crates/myrn-engine/src/learn.rs#learn::Engine::mark_used_files@0579646529b014b5
map_version: '2'
---
# Engine::mark_used_files

Sets used = 1 on not-yet-used injections in the given session whose item is at one of the given repo-relative paths or is anchored to a symbol at one of those paths. Returns the number of rows changed.

## Behaviour

- empty files list -> 0 and no writes
- injection already marked used -> not counted again, since the update filters on used = 0
- injections from other sessions are not changed

## Terms

file read marks used, file edit usage, anchored record, touched file, repo-relative path, session file access, used by file

## Connections

- **Location:** `crates/myrn-engine/src/learn.rs:87-101`
- **Signature:** `pub fn mark_used_files(&self, session: &str, files: &[String]) -> Result<usize>`
- **In:** [[file-crates-myrn-engine-src-learn-rs]]
- **Called by:** [[hook-handle]]
- **Tests:** `opening_the_file_or_asking_about_it_counts_as_use`, `learned_ranking_moves_items_by_past_use_and_fixed_ignores_it`
