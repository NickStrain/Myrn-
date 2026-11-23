---
id: engine-publish
type: summary
status: unverified
title: Engine::publish
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::Engine::publish
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::Engine::publish@79abc67850e69b80
map_version: '2'
---
# Engine::publish

Writes each planned item to its knowledge path through a temp file and rename, deletes the private source file when the paths differ, keeps local verified status, then runs sync. Nothing is committed to git; the files are left for git add.

## Behaviour

- write goes to a .tmp file, then renames to the target, so a partial file is not left at the target
- verified_here item -> set_local_verified runs on the re-parsed text with a verified-here note
- creates missing parent directories before writing

## Terms

write, move, atomic write, git add, sync, commit, publish to knowledge

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:174-206`
- **Signature:** `pub fn publish(&mut self, id: &str) -> Result<PublishPlan>`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
- **Calls:** [[engine-note-roots]], [[sym-parent]], [[engine-publish-plan]], [[frontmatter-parse]], [[layout-folder-for]], [[layout-safe-stem]], [[sync-sync]], [[sync-set-local-verified]]
- **Called by:** [[publish-run]]
- **Tests:** `published_record_reaches_a_teammate_through_git`, `imported_records_start_unverified_whatever_they_say`, `shared_records_that_disagree_are_flagged_for_a_person`, `a_teammate_verifies_a_shared_record_locally_without_changing_the_file`
