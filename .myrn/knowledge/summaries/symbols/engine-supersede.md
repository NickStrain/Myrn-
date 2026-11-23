---
id: engine-supersede
type: summary
status: unverified
title: Engine::supersede
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::supersede
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::supersede@d95d4dcf27ff9844
map_version: '2'
---
# Engine::supersede

Stores a corrected record that links back to the old id with a supersedes relation; the old record is kept and marked superseded by sync.

## Behaviour

- Unknown id -> error, item not a JSON object -> error
- Item with an existing relations array -> supersedes entry is appended
- Item without relations -> a relations array holding only the supersedes entry is created

## Terms

replace, correction, replacement, obsolete record

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:165-181`
- **Signature:** `pub fn supersede(&mut self, id: &str, item: &serde_json::Value) -> Result<RecordOutcome>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Calls:** [[dedupe-find-id]], [[engine-record-json]]
- **Tests:** `supersede_replaces_and_keeps_history`
