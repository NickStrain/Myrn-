---
id: engine-record-row
type: summary
status: unverified
title: Engine::record_row
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::record_row
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::record_row@fdb09e14f4fa36e2
map_version: '2'
---
# Engine::record_row

Loads one record by id from the notes table, preferring the shared copy when several visibilities exist, and returns its row with the parsed note.

## Behaviour

- unknown id -> error `no record ...; see myrn log`
- shared copy wins over private copy for the same id
- data that is not valid Note JSON -> error, unlike `records`, which skips it

## Terms

lookup, load record, fetch note, shared copy, private copy

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:164-195`
- **Signature:** `fn record_row(&self, id: &str) -> Result<(RecordRow, Note)>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[graph-node-graph]], [[note-display-title]]
- **Called by:** [[engine-trace]], [[engine-forget-plan]]
