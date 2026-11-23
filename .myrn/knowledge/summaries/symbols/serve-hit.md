---
id: serve-hit
type: summary
status: unverified
title: serve::Hit
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::serve::Hit
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::serve::Hit@7765976ccb6a2216
map_version: '2'
---
# serve::Hit

JSON row for one search result, with id, label, type and location. serve builds a list of these from engine.find.

## Behaviour

- field ntype is serialized as 'type'
- label is display_name() and location is location() of the node
- search returns at most 30 hits

## Terms

search result, search hit, node summary, match row, result item

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:149-155`
- **Signature:** `struct Hit`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
