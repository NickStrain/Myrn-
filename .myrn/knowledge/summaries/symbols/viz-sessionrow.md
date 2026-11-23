---
id: viz-sessionrow
type: summary
status: unverified
title: viz::SessionRow
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::SessionRow
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::SessionRow@2d2b2a9c0eb8402d
map_version: '2'
---
# viz::SessionRow

One recorded session for the UI: episode fields flattened in, plus outcome, summary, and injected and used counts.

## Behaviour

- injected and used are 0 when the session has no usage rows
- episode fields appear at the same JSON level, not nested
- outcome and summary come from the episode

## Terms

episode, session log, outcome, usage totals, injected records

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:99-106`
- **Signature:** `pub struct SessionRow`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
