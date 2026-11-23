---
id: engine-promote
type: summary
status: unverified
title: Engine::promote
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::promote
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::promote@e7f1b41df6af006b
map_version: '2'
---
# Engine::promote

Promotes unverified earned records whose evidence holds: a cited test passed in a recorded session, or the fixing commit is in the branch history and not reverted. Suspect and superseded records are excluded.

## Behaviour

- No candidates -> empty vec, no history walk
- Shared knowledge-file record -> only a local verification is set, committed file untouched, relink requested
- Other record -> status set to verified and the note is rewritten

## Terms

verify, promotion, verified status, evidence check, earned record

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:186-240`
- **Signature:** `pub fn promote(&mut self) -> Result<Vec<Promotion>>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Calls:** [[engine-note-roots]], [[dedupe-find-id]], [[engine-put-note]], [[engine-passing-test-runs]], [[engine-verified-by-test]], [[engine-history-messages]], [[states-fix-landed]], [[states-is-shared-key]], [[graph-nodes-where]], [[sync-sync]], [[sync-set-local-verified]]
- **Called by:** [[hook-handle]], [[verify-run]]
- **Tests:** `a_teammate_verifies_a_shared_record_locally_without_changing_the_file`, `records_become_verified_when_a_cited_test_passes_or_the_fix_lands`, `a_reverted_fix_does_not_verify`
