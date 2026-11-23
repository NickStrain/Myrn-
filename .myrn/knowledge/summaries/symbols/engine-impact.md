---
id: engine-impact
type: summary
status: unverified
title: Engine::impact
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::impact
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::impact@42fe46808cc01fcc
map_version: '2'
---
# Engine::impact

Reports what depends on a symbol or file: direct and indirect callers, importers, direct and indirect tests, and the number of files affected.

## Behaviour

- For a file, members are the symbols defined in it; for a symbol, members are its members and importers come from its file
- Callers that are themselves targets are skipped
- The caller walk runs with depth 3 and limit 300 as passed to walks::callers, and truncated is passed through

## Terms

blast radius, dependents, callers, reverse dependencies, affected tests, change impact

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:534-600`
- **Signature:** `pub fn impact(&self, query: &str) -> Result<Lookup<ImpactReport>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-pick]], [[engine-links]], [[link-new]], [[walks-members]], [[walks-direct-tests]], [[walks-callers]], [[id-file-id]], [[graph-neighbors]]
- **Called by:** [[impact]], [[impact-call]], [[query-impact]]
