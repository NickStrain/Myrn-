---
id: engine-summary-is-current
type: summary
status: unverified
title: Engine::summary_is_current
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::summary_is_current
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::summary_is_current@d10a47e3a66d47d7
---
# Engine::summary_is_current

Returns true only when the node has a summary and summary_outdated is false.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:535-540`
- **Signature:** `pub(crate) fn summary_is_current(&self, n: &graph::Node) -> Result<bool>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[engine-summary-outdated]]
- **Called by:** [[engine-records-request]]
