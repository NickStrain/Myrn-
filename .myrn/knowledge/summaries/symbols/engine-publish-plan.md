---
id: engine-publish-plan
type: summary
status: unverified
title: Engine::publish_plan
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::Engine::publish_plan
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::Engine::publish_plan@adebd64b50164f5e
map_version: '2'
---
# Engine::publish_plan

Checks that the record can be published and builds the exact file contents for it and any private root cause, without writing anything. It fails with an error for unknown, already shared, superseded, suspect, or empty-after-cleaning records.

## Behaviour

- unknown id -> error saying no record, pointing to myrn log
- status suspect -> error suggesting myrn verify --confirm or supersede
- non-inverse CausedBy link to a private note -> that note is added as a second item

## Terms

dry run, preview, eligibility, checks, root cause, suspect, superseded, already shared

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:67-170`
- **Signature:** `pub fn publish_plan(&self, id: &str) -> Result<PublishPlan>`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
- **Calls:** [[dedupe-find-id]], [[engine-note-file]], [[engine-session-agent]], [[engine-publisher]], [[sharing-clean]], [[sharing-today]], [[states-is-shared-key]], [[graph-node-graph]], [[note-extra-str]], [[note-set-extra]], [[frontmatter-render]], [[layout-folder-for]], [[layout-file-name]], [[layout-knowledge-dir]]
- **Called by:** [[engine-publish]], [[publish-run]]
- **Tests:** `published_record_reaches_a_teammate_through_git`
