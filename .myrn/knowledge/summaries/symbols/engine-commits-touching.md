---
id: engine-commits-touching
type: summary
status: unverified
title: Engine::commits_touching
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::commits_touching
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::commits_touching@f8f2678680b0f601
map_version: '2'
---
# Engine::commits_touching

Returns up to limit commits that touched a file or symbol, newest first.

## Behaviour

- File: commits from the commit_files table matching its path, ordered by commit time in SQL
- Symbol: commits joined by touches edges, sorted by time and truncated to limit
- Link name is the commit's qualified name and location is its title

## Terms

commit list, recent commits, touches edges, file commits

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:400-437`
- **Signature:** `fn commits_touching(&self, n: &Node, limit: usize) -> Result<Vec<Link>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[graph-node-graph]], [[graph-neighbors]]
- **Called by:** [[engine-symbol-card]], [[engine-file-card]]
