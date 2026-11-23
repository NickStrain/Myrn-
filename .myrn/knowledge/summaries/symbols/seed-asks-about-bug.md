---
id: seed-asks-about-bug
type: summary
status: unverified
title: seed::asks_about_bug
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::asks_about_bug
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::asks_about_bug@5d9c38a0ffa73225
map_version: '2'
---
# seed::asks_about_bug

Decides whether a task reads as a bug report (behaviour that is wrong) rather than a question or a request for new code. Matching is by substring on the lowercased text.

## Behaviour

- chunk drops the last chunk -> true (drop matches)
- Add a function ... it returns every nth item -> false (starts with add)
- Which module defines X, and what does it do? -> false (question with no fix)

## Terms

bug report, regression, wrong behaviour, defect, unexpected output, failing code, question versus bug

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:319-349`
- **Signature:** `pub fn asks_about_bug(task: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[seed-is-question]]
- **Called by:** [[engine-context-excluding]], [[seed-seeds]]
- **Tests:** `bug_reports_are_told_apart`
