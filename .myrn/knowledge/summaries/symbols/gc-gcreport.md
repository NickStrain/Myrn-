---
id: gc-gcreport
type: summary
status: unverified
title: gc::GcReport
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::GcReport
anchors:
- sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::GcReport@3cf3e4bd1d5589ab
map_version: '2'
---
# gc::GcReport

Holds the counts and key lists a gc run produced, plus database size before and after. Duplicate copies that differ are listed, not removed.

## Behaviour

- Default -> dry_run false, all counts 0, all vecs empty
- duplicates_differing -> keys of copies whose body, links or anchors differ from the indexed copy; file kept
- dry_run true -> bytes_after equals bytes_before because nothing was written

## Terms

cleanup summary, gc result, dry run output, space freed, bytes reclaimed, removed duplicates, gc stats

## Connections

- **Location:** `crates/myrn-engine/src/validity/gc.rs:15-28`
- **Signature:** `pub struct GcReport`
- **In:** [[file-crates-myrn-engine-src-validity-gc-rs]]
