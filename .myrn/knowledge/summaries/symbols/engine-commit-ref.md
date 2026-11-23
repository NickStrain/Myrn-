---
id: engine-commit-ref
type: summary
status: unverified
title: Engine::commit_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::commit_ref
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::commit_ref@808d11e735204c4c
map_version: '2'
---
# Engine::commit_ref

Looks up a commit node whose id starts with the given sha prefix, and returns a Ref only when exactly one commit matches.

## Behaviour

- one match on the prefix -> Ref with a 12-char header and anchor None
- two or more matches -> None, so an ambiguous prefix is rejected
- no match -> None

## Terms

commit lookup, sha prefix, ambiguous commit, indexed history, commit node

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:70-86`
- **Signature:** `fn commit_ref(&self, sha: &str) -> Result<Option<Ref>>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[graph-nodes-where]]
- **Called by:** [[engine-resolve-ref]], [[engine-check-evidence]]
