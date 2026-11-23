---
id: engine-check-evidence
type: summary
status: unverified
title: Engine::check_evidence
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::check_evidence
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::check_evidence@7ea3fba2419e56e5
map_version: '2'
---
# Engine::check_evidence

Checks each evidence item and returns the valid evidence, the code refs it points to, and a problem message for each invalid item.

## Behaviour

- an item that is not exactly one kind: value pair -> problem listing the allowed kinds
- kind test that resolves to a node other than test or file -> problem
- kind symbol or test that resolves to a commit -> problem asking for a commit kind

## Terms

evidence item, evidence kind, test_run, validation problem, valid evidence, kind value pair

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:208-276`
- **Signature:** `pub(crate) fn check_evidence( &self, items: &[BTreeMap<String, String>], ) -> Result<(Vec<Evidence>, Vec<Ref>, Vec<String>)>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Calls:** [[kind-as-str]], [[engine-commit-ref]], [[engine-resolve-ref]], [[engine-recorded-test-run]]
- **Called by:** [[engine-record]]
