---
id: engine-viz-records
type: summary
status: unverified
title: Engine::viz_records
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_records
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_records@c28ab4678a3ce58b
map_version: '2'
---
# Engine::viz_records

Loads up to 2000 learned records with injection and use counts, drops summary-sourced ones, and attaches sync conflicts.

## Behaviour

- the summary-source filter runs after the 2000 cap, so fewer than 2000 may be returned
- counts are summed per note id from usage; no usage rows gives 0 and 0
- conflicts come from sync::conflicts

## Terms

learned records, usage counts, injection count, sync conflicts, summary filter, record cap

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:626-660`
- **Signature:** `pub fn viz_records(&self) -> Result<RecordsData>`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Calls:** [[engine-records]], [[sync-conflicts]]
- **Tests:** `notes_graph_map_records_and_details`
