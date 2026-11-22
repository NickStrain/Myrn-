---
id: impact
type: summary
status: unverified
title: impact
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#impact
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#impact@5d4fb7fe9b7d781d
---
# impact

Calls e.impact(id) and returns the ImpactReport. Panics with the JSON-serialized result if the lookup is not Found.

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:79-87`
- **Signature:** `fn impact(e: &Engine, id: &str) -> ImpactReport`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[engine-impact]]
- **Called by:** [[check]]
- **Tests:** `gate0_rust`
