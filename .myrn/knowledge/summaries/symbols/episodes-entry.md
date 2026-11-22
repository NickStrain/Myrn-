---
id: episodes-entry
type: summary
status: unverified
title: episodes::Entry
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::Entry
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::Entry@90b2135b15ea6409
---
# episodes::Entry

Serde record for one normalized event, with required t, agent, and event fields and optional fields for prompt text, tool, outcome, command, error, files, symbols, test result, reason, via, and commits. Unset options and empty vecs are omitted from the JSON. The struct does no redaction; the engine redacts text fields before they reach it.

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:15-53`
- **Signature:** `pub struct Entry`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
