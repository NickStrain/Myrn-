---
id: engine-commit-ref
type: summary
status: unverified
title: Engine::commit_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::commit_ref
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::commit_ref@808d11e735204c4c
---
# Engine::commit_ref

Finds the commit node whose id starts with the given sha, case-insensitively, and returns a Ref only when exactly one commit matches; zero or multiple matches return None.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:70-86`
- **Signature:** `fn commit_ref(&self, sha: &str) -> Result<Option<Ref>>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[graph-nodes-where]]
- **Called by:** [[engine-resolve-ref]], [[engine-check-evidence]]
