---
id: engine-note-issues
type: summary
status: unverified
title: Engine::note_issues
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::note_issues
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::note_issues@89af29f518701e94
---
# Engine::note_issues

Returns up to limit note problems: malformed headers, dangling links and duplicate ids.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:306-308`
- **Signature:** `pub fn note_issues(&self, limit: usize) -> Result<Vec<sync::Issue>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[sync-issues]]
- **Called by:** [[engine-doctor]]
