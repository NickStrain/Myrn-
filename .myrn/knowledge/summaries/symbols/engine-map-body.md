---
id: engine-map-body
type: summary
status: unverified
title: Engine::map_body
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::Engine::map_body
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::Engine::map_body@b5ee2f40a9401b60
---
# Engine::map_body

Builds the Markdown body for one summary note: a title heading, the summary text, then a Connections list of `[[links]]` to related notes (module parent/children, file defines/imports/imported-by, symbol location, signature, calls/called-by, and up to 6 tests), each capped at 25 links per kind.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:483-588`
- **Signature:** `fn map_body( &self, e: &Entry, summary: &str, notes: &HashMap<String, String>, ) -> Result<String>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[episode-clip]], [[walks-direct-tests]], [[summarize-title-of]], [[ids]], [[id-file-id]], [[id-module-id]], [[id-parent-dir]], [[node-location]], [[graph-nodes-where]], [[graph-neighbors]]
- **Called by:** [[engine-summarize]]
