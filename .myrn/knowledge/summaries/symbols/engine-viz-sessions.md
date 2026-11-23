---
id: engine-viz-sessions
type: summary
status: unverified
title: Engine::viz_sessions
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_sessions
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_sessions@7cf2abdfec459e2d
map_version: '2'
---
# Engine::viz_sessions

Lists recorded sessions through episodes(limit), each with outcome, summary and summed injected and used counts.

## Behaviour

- sessions with no usage rows report injected 0 and used 0
- usage rows with no session id are summed under an empty key
- order and count come from episodes(); the doc comment says newest first

## Terms

episodes, session list, outcome, usage totals, recent sessions

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:663-695`
- **Signature:** `pub fn viz_sessions(&self, limit: usize) -> Result<Vec<SessionRow>>`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Calls:** [[engine-episodes]], [[episode-outcome]], [[episode-summary]]
- **Tests:** `notes_graph_map_records_and_details`
