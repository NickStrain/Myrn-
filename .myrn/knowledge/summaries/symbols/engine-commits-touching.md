---
id: engine-commits-touching
type: summary
status: unverified
title: Engine::commits_touching
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::commits_touching
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::commits_touching@f8f2678680b0f601
---
# Engine::commits_touching

Returns up to limit commits for a file (from commit_files, newest first) or a symbol (from touches edges, sorted by commit time, newest first). Each Link takes its name from the commit qname and its location from the commit title.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:375-412`
- **Signature:** `fn commits_touching(&self, n: &Node, limit: usize) -> Result<Vec<Link>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[graph-node-graph]], [[graph-neighbors]]
- **Called by:** [[engine-symbol-card]], [[engine-file-card]]
