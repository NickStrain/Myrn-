---
id: summarize-dirjob
type: summary
status: unverified
title: summarize::DirJob
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::DirJob
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::DirJob@d4da650c7a9e81ee
map_version: '2'
---
# summarize::DirJob

One directory to summarize: module node id, path (empty for the project root) and listing lines of its children's summaries.

## Behaviour

- listing line is '- dir name/: summary' or '- file name: summary', with '(no summary)' for a child that has none
- a job is made when the directory has no summary or its children hash changed, and at least one child has a summary
- empty path marks the project root job

## Terms

directory job, dir request, child listing, rollup input

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:686-690`
- **Signature:** `struct DirJob`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
