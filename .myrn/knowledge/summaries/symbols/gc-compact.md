---
id: gc-compact
type: summary
status: unverified
title: gc::compact
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::compact
anchors:
- sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::compact@98d1fd35421b09e6
map_version: '2'
---
# gc::compact

Keeps session start and end, the first prompt, edits, failures, test results and commit entries, and folds the file lists of dropped tool entries into one synthetic read line appended at the end. Returns the input unchanged if nothing would be dropped.

## Behaviour

- Two tool entries with files [a.rs] and [a.rs, b.rs], no keep flag -> one appended line with files [a.rs, b.rs] in first-seen order
- Later prompts are dropped and their text is not kept; only the first prompt survives
- Empty input -> empty vec; a single read-only entry -> returned unchanged

## Terms

log trimming, session log pruning, keep important entries, fold reads, read summary, shrink log, first prompt only

## Connections

- **Location:** `crates/myrn-engine/src/validity/gc.rs:142-187`
- **Signature:** `fn compact(entries: &[Entry]) -> Vec<Entry>`
- **In:** [[file-crates-myrn-engine-src-validity-gc-rs]]
- **Called by:** [[engine-gc]]
- **Tests:** `compaction_keeps_what_matters`
