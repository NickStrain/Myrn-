---
id: episode-persist
type: summary
status: unverified
title: episode::persist
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::persist
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::persist@31cc18c5e51431f9
map_version: '2'
---
# episode::persist

Upserts the episode node with its status and summary, then replaces its touches edges with edges to symbols and edited files that exist in the graph.

## Behaviour

- title is the first line of the task clipped to 100 chars; no task -> title is null
- symbol touches are created only if the symbol node exists; files edited with no changed symbol get a file touch if the file node exists; read-only files get no edge
- an edited file that contains a touched symbol gets no file edge, only the symbol edge; produced edges are left alone

## Terms

graph write, store, save, sync, upsert, link files, edges

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:153-203`
- **Signature:** `pub fn persist(conn: &Connection, ep: &Episode) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Calls:** [[episode-outcome]], [[episode-summary]], [[episode-clip]], [[id-file-id]], [[id-episode-id]], [[graph-upsert-node]], [[graph-insert-edge]], [[graph-node-graph]]
- **Called by:** [[engine-capture]], [[engine-reindex-episodes]]
