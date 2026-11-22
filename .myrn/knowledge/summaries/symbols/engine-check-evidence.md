---
id: engine-check-evidence
type: summary
status: unverified
title: Engine::check_evidence
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::check_evidence
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::check_evidence@7ea3fba2419e56e5
---
# Engine::check_evidence

Validates a list of single-pair evidence maps, returning three things: the accepted Evidence entries, the resolved Refs, and a problem message for each rejected item. A test kind must resolve to a test or file node, a commit listed under symbol or test is rejected in favor of the commit kind, and a test_run must match a command recorded in recent sessions.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:208-276`
- **Signature:** `pub(crate) fn check_evidence( &self, items: &[BTreeMap<String, String>], ) -> Result<(Vec<Evidence>, Vec<Ref>, Vec<String>)>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[kind-as-str]], [[engine-commit-ref]], [[engine-resolve-ref]], [[engine-recorded-test-run]]
- **Called by:** [[engine-record]]
