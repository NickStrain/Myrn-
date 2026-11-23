---
id: knowledge-tracedcommit
type: summary
status: unverified
title: knowledge::TracedCommit
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::TracedCommit
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::TracedCommit@134f0cecae23c433
map_version: '2'
---
# knowledge::TracedCommit

One Git commit tied to a record: a cited commit (rel is `fixed in` or `evidence`) or a commit that touched the note file (rel is `added`, `changed` or `removed`).

## Behaviour

- a cited sha that does not resolve to a commit is skipped silently
- sha is the full commit id and subject is the first line of the message
- time is the commit time in unix seconds

## Terms

commit, sha, citation, history entry, git commit

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:62-67`
- **Signature:** `pub struct TracedCommit`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
