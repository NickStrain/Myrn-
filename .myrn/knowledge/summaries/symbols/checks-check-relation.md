---
id: checks-check-relation
type: summary
status: unverified
title: checks::check_relation
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::check_relation
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::check_relation@c9fe51974f680b1d
---
# checks::check_relation

Returns an error message listing AGENT_RELATIONS if the relation is not allowed for agents, otherwise None.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:41-48`
- **Signature:** `pub fn check_relation(rel: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Called by:** [[engine-record]]
