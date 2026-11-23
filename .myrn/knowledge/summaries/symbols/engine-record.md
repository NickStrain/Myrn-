---
id: engine-record
type: summary
status: unverified
title: Engine::record
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record@eab8717ea153a9e2
map_version: '2'
---
# Engine::record

Validates a RecordItem against schema rules and reference resolution, and rejects it with nothing written if any problem is found. Otherwise it writes the root cause and the main note, merges or dedupes against existing notes, marks superseded notes, and syncs the index.

## Behaviour

- body under 12 characters after trim, an unresolved reference, a type-restricted field on the wrong type, or empty evidence -> rejected, nothing written
- summary needs exactly one about target of type symbol, test, file or module; empty evidence is then filled with that target
- fact or decision about the same targets as a contradicting note -> Contradicts link and conflict message, still accepted; a supersedes relation marks the target superseded

## Terms

validate, dedupe, merge, contradiction, supersede, root cause, evidence check, save

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:64-382`
- **Signature:** `pub fn record(&mut self, item: RecordItem) -> Result<RecordOutcome>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[kind-as-str]], [[engine-note-roots]], [[episode-clip]], [[checks-check-type]], [[checks-check-relation]], [[engine-resolve-ref]], [[engine-check-evidence]], [[contradiction-find]], [[dedupe-find-bug]], [[dedupe-find-summary]], [[dedupe-find-id]], [[dedupe-merge]], [[recordoutcome-reject]], [[engine-put-note]], [[engine-current-session]], [[record-link]], [[symbols-resolve-all]], [[engine-session-agent]], [[sharing-today]], [[relation-parse]], [[note-set-extra]], [[note-new]], [[layout-make-id]], [[sync-sync]]
- **Called by:** [[engine-record-json]]
