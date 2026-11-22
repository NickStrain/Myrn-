---
id: episode-persist
type: summary
status: unverified
title: episode::persist
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::persist
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::persist@31cc18c5e51431f9
---
# episode::persist

Upserts the episode node (id from episode_id(session), origin 'earned', title from the first line of the task clipped to 100 chars, status from outcome, full JSON in extra), then deletes this episode's existing 'touches' edges and reinserts them. Symbols are linked only if they exist in the graph; an edited file is linked directly only if no touched symbol belongs to it and its file node exists. 'produced' edges are not modified.

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:153-203`
- **Signature:** `pub fn persist(conn: &Connection, ep: &Episode) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Calls:** [[episode-outcome]], [[episode-summary]], [[episode-clip]], [[id-file-id]], [[id-episode-id]], [[graph-upsert-node]], [[graph-insert-edge]], [[graph-node-graph]]
- **Called by:** [[engine-capture]], [[engine-reindex-episodes]]
