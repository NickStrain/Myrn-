---
id: episode-episode
type: summary
status: unverified
title: episode::Episode
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode@6757464b81fc4455
map_version: '2'
---
# episode::Episode

Holds the per-session episode record: task, counts, file, symbol, command, error, test and commit collections. persist serializes it as JSON into the node's extra field.

## Behaviour

- Default value has empty collections, zero counts, and None for ended, task and last_test
- ended and last_test stay None until a session_end entry or a test-tagged tool entry is folded
- files_read, files_edited and symbols_touched are BTreeSets, so they serialize in sorted order

## Terms

session record, run record, session state, per-session data, task log, episode data

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:20-38`
- **Signature:** `pub struct Episode`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
