---
id: episodes-entry
type: summary
status: unverified
title: episodes::Entry
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::Entry
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::Entry@90b2135b15ea6409
map_version: '2'
---
# episodes::Entry

Serde record for one normalized agent event. Optional fields and empty vectors are omitted when serialized and default when missing on deserialize.

## Behaviour

- t is Unix seconds; agent is 'claude' or 'codex'; event is 'session_start', 'prompt', 'tool', or 'session_end'
- None and empty files/symbols/commits are left out of the JSON line
- No redaction happens here; text fields arrive already redacted by the engine

## Terms

record, log line, event, episode entry, session event, journal row

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:15-53`
- **Signature:** `pub struct Entry`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
