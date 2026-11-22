---
id: engine-impact
type: summary
status: unverified
title: Engine::impact
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::impact
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::impact@42fe46808cc01fcc
---
# Engine::impact

Resolves a query to a symbol or file and reports its members, direct and indirect callers (depth 3, up to 300 nodes, target excluded), importers, direct and indirect tests, the number of affected files, and whether the walk was truncated.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:505-571`
- **Signature:** `pub fn impact(&self, query: &str) -> Result<Lookup<ImpactReport>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-pick]], [[engine-links]], [[link-new]], [[walks-members]], [[walks-direct-tests]], [[walks-callers]], [[id-file-id]], [[graph-neighbors]]
- **Called by:** [[impact]], [[impact-call]], [[query-impact]]
