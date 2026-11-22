---
id: engine-summary-outdated
type: summary
status: unverified
title: Engine::summary_outdated
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::summary_outdated
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::summary_outdated@d97b040996ed4378
---
# Engine::summary_outdated

Returns true only when the node has both a summary and a body hash, and at least one summary anchored to the node has a hash that is not a prefix of the current body hash; it returns false when there is no summary, no body hash, or no anchored summary hashes.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:543-555`
- **Signature:** `pub fn summary_outdated(&self, n: &graph::Node) -> Result<bool>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Called by:** [[engine-symbol-card]], [[engine-summary-is-current]]
