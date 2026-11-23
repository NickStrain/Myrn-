---
id: outcome-commits-since
type: summary
status: unverified
title: outcome::commits_since
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/outcome.rs#capture::outcome::commits_since
anchors:
- sym:crates/myrn-engine/src/capture/outcome.rs#capture::outcome::commits_since@863107c8e79b0af8
map_version: '2'
---
# outcome::commits_since

Walks commits from HEAD in the repository found from root, time-sorted, and returns up to limit entries of 'sha summary' for commits with time >= since, newest first.

## Behaviour

- root not in a git repo, or HEAD unborn -> empty Vec with no error; since=0 or limit=0 -> empty Vec
- Walk stops at the first commit older than since, so a commit with an earlier timestamp than an older one can be missed if timestamps are out of order
- Commit with no summary -> entry is just the sha (trim_end removes trailing space); revwalk errors on individual commits are skipped silently

## Terms

recent commits, commit history, git log, commits since timestamp, session commits, commit list, head history, revwalk

## Connections

- **Location:** `crates/myrn-engine/src/capture/outcome.rs:62-84`
- **Signature:** `pub fn commits_since(root: &Path, since: i64, limit: usize) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-capture-outcome-rs]]
- **Calls:** [[episode-summary]]
- **Called by:** [[engine-capture]]
