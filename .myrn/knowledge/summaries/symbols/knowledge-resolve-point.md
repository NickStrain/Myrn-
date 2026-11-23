---
id: knowledge-resolve-point
type: summary
status: unverified
title: knowledge::resolve_point
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::resolve_point
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::resolve_point@8248fe39a8118ca0
map_version: '2'
---
# knowledge::resolve_point

Turns a date (`YYYY-MM-DD`) or a revision (`HEAD~3`, tag, sha) into a commit; a date means the newest HEAD commit at or before the end of that day in UTC.

## Behaviour

- `2026-11-01` -> first commit walked from HEAD with time at or before 2026-11-01 23:59:59 UTC
- date with no commit that early -> error `no commit on or before <date>`
- string that is neither a date nor a revision -> error mentioning YYYY-MM-DD

## Terms

revision, commit lookup, date, rev, sha, tag, commit at date

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:503-520`
- **Signature:** `fn resolve_point<'r>(repo: &'r git2::Repository, at: &str) -> Result<git2::Commit<'r>>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[knowledge-date-end]]
- **Called by:** [[engine-knowledge-at]]
