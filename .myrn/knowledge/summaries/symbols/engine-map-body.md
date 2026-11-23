---
id: engine-map-body
type: summary
status: unverified
title: Engine::map_body
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::Engine::map_body
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::Engine::map_body@aaafdf61a8981c60
map_version: '2'
---
# Engine::map_body

Builds the Markdown body of one summary note: a heading, the summary text, optional Behaviour and Terms sections, then a Connections list of Obsidian [[links]] to other notes.

## Behaviour

- file rows: Location, In, Defines, Imports, Imported by; symbol rows: Location, Signature (clipped to 200 chars), In, Calls, Called by, Tests (max 6 titles); directory rows: In, Directories, Files
- links point only at targets that already have a summary; each row takes at most 25 links with duplicates removed
- empty rows are left out, so the Connections section is omitted when no row has content

## Terms

note body, wiki links, Obsidian links, connections section, backlinks, markdown note

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:508-622`
- **Signature:** `fn map_body( &self, e: &Entry, summary: &str, notes: &HashMap<String, String>, ) -> Result<String>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[episode-clip]], [[walks-direct-tests]], [[summarize-title-of]], [[ids]], [[id-file-id]], [[id-module-id]], [[id-parent-dir]], [[node-location]], [[graph-nodes-where]], [[graph-neighbors]]
- **Called by:** [[engine-summarize]]
