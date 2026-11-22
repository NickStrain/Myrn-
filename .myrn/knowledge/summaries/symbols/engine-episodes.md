---
id: engine-episodes
type: summary
status: unverified
title: Engine::episodes
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::episodes
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::episodes@887172043a796842
---
# Engine::episodes

Returns up to limit non-empty episode logs folded into Episodes, newest started first.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:977-990`
- **Signature:** `pub fn episodes(&self, limit: usize) -> Result<Vec<Episode>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[episode-fold]], [[episodes-read]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[log-run]]
- **Tests:** `session_end_asks_once_per_batch_of_edits`, `shell_edits_are_captured_and_lead_to_a_record_request`
