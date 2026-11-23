---
id: engine-summary-is-current
type: summary
status: unverified
title: Engine::summary_is_current
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::summary_is_current
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::summary_is_current@d10a47e3a66d47d7
map_version: '2'
---
# Engine::summary_is_current

Reports whether a node has a summary that matches its current body. It requires a summary to exist and negates summary_outdated.

## Behaviour

- node with no summary -> false
- summary present and summary_outdated false -> true
- summary present but summary_outdated true -> false

## Terms

summary freshness, up to date, stale summary, body hash

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:546-551`
- **Signature:** `pub(crate) fn summary_is_current(&self, n: &graph::Node) -> Result<bool>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[engine-summary-outdated]]
- **Called by:** [[engine-records-request]]
