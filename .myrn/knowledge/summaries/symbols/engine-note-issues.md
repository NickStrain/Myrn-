---
id: engine-note-issues
type: summary
status: unverified
title: Engine::note_issues
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::note_issues
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::note_issues@89af29f518701e94
map_version: '2'
---
# Engine::note_issues

Returns up to limit problems found in the notes: malformed headers, dangling links and duplicate ids.

## Behaviour

- limit caps the number of issues returned
- Reads from the index, so the notes must have been synced

## Terms

note errors, lint, broken links, duplicate ids, malformed note

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:314-316`
- **Signature:** `pub fn note_issues(&self, limit: usize) -> Result<Vec<sync::Issue>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[sync-issues]]
- **Called by:** [[engine-doctor]]
