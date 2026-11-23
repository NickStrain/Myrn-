---
id: impactreport-render
type: summary
status: unverified
title: ImpactReport::render
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::ImpactReport::render
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::ImpactReport::render@bf2903b92cb8b1ac
map_version: '2'
---
# ImpactReport::render

Produces the blast-radius text for a target: header, member count, direct and indirect callers with hop counts, importers, tests with hop tags, and a summary line with counts and a truncation note. The tests list always appears, showing none found when empty.

## Behaviour

- indirect callers print as "name [N hops]"; indirect tests print as "name [via N hops]"
- summary counts direct callers, indirect callers, combined tests and files_affected as given
- truncated true -> summary ends with "(walk truncated)"

## Terms

impact text, blast radius report, change impact view, hop count, affected summary

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:288-340`
- **Signature:** `pub fn render(&self, limit: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[cards-row]], [[cards-list]]
