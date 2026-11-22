---
id: impactreport-render
type: summary
status: unverified
title: ImpactReport::render
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::ImpactReport::render
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::ImpactReport::render@bf2903b92cb8b1ac
---
# ImpactReport::render

Renders the impact report: header with target name and location, a member count note, direct callers, indirect callers labeled with hop count, importers, and a combined tests list where indirect entries show 'via N hops'. The summary line gives counts and appends '(walk truncated)' when truncated is set.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:281-333`
- **Signature:** `pub fn render(&self, limit: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[cards-row]], [[cards-list]]
