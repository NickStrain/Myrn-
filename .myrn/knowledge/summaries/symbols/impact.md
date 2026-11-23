---
id: impact
type: summary
status: unverified
title: impact
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#impact
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#impact@5d4fb7fe9b7d781d
map_version: '2'
---
# impact

Calls engine impact on an id and returns the ImpactReport, panicking if the result is not found.

## Behaviour

- Found(report) -> the ImpactReport with direct and indirect callers and tests
- any other Lookup variant -> panic with the JSON of the result
- unknown id -> panic

## Terms

impact report, blast radius, affected callers, affected tests

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:79-87`
- **Signature:** `fn impact(e: &Engine, id: &str) -> ImpactReport`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[engine-impact]]
- **Called by:** [[check]]
- **Tests:** `gate0_rust`
