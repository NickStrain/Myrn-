---
id: episode-episode
type: summary
status: unverified
title: episode::Episode
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode@6757464b81fc4455
---
# episode::Episode

Per-session record with task (first prompt), prompt count, files_read and files_edited sets, symbols_touched, commands and errors (capped), tests_passed and tests_failed counts, last_test, commits, and started/ended timestamps. Derives Serialize; persist stores the full JSON in the node's extra field.

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:20-38`
- **Signature:** `pub struct Episode`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
