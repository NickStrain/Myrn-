---
id: engine-changed
type: summary
status: unverified
title: Engine::changed
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::changed
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::changed@52094a307366044b
map_version: '2'
---
# Engine::changed

Builds one Changed entry for a drift item, resolving its symbol node when it exists.

## Behaviour

- No symbol_id -> node None, name '(code no longer found)', gone true
- Node found -> name from display_name, location set, gone from current_hash

## Terms

drift item, anchor resolution, symbol lookup

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:82-104`
- **Signature:** `fn changed(&self, d: &Drift) -> Result<Changed>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Calls:** [[node-location]], [[graph-node-graph]]
- **Called by:** [[engine-suspects]]
