---
id: states-changed
type: summary
status: unverified
title: states::Changed
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Changed
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Changed@382a66171c3bd509
map_version: '2'
---
# states::Changed

Describes one drifted anchor: the symbol node id if known, its name, its location, and whether the code is gone.

## Behaviour

- Symbol missing from the graph -> node None, name is the symbol id or '(code no longer found)', gone true
- Node found -> gone is true only when current_hash is None

## Terms

anchor drift, removed code, deleted symbol, changed code

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:40-46`
- **Signature:** `pub struct Changed`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
