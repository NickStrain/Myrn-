---
id: cards-impactreport
type: summary
status: unverified
title: cards::ImpactReport
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::ImpactReport
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::ImpactReport@8a348f59bdc3479c
map_version: '2'
---
# cards::ImpactReport

Holds the blast radius of a target symbol: direct and indirect callers with hop counts, importers, direct and indirect tests, the number of affected files, and whether the graph walk was truncated.

## Behaviour

- indirect_callers entries carry a distance in hops, two or more
- truncated true -> render adds "(walk truncated)" to the summary row
- members non-empty -> render notes the member count

## Terms

blast radius, change impact, dependents, transitive callers, walk cutoff, affected files

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:87-98`
- **Signature:** `pub struct ImpactReport`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
