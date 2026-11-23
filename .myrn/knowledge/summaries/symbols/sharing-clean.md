---
id: sharing-clean
type: summary
status: unverified
title: sharing::clean
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::clean
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::clean@90a060547e43df84
map_version: '2'
---
# sharing::clean

Strips directive-shaped text from the body, title and evidence, and rewrites test_run evidence to its test step and local paths to relative or hidden form. Returns two lists: what was stripped and what evidence was trimmed.

## Behaviour

- body sentence that is a directive -> removed from body and added to stripped
- title that is a directive -> title set to None and added to stripped
- test_run evidence -> reduced to the test command, and trimmed records 'kind: before -> after'

## Terms

sanitize, scrub, redact, strip directives, prompt injection, evidence trim, remove instructions

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:222-254`
- **Signature:** `fn clean(note: &mut Note, root: &Path) -> (Vec<String>, Vec<String>)`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
- **Calls:** [[checks-test-step]], [[sharing-local-paths]], [[directives-is-directive]], [[directives-strip]]
- **Called by:** [[engine-publish-plan]]
- **Tests:** `clean_strips_body_title_and_evidence`
