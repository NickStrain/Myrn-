---
id: states-suspectrecord
type: summary
status: unverified
title: states::SuspectRecord
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::SuspectRecord
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::SuspectRecord@a3e22f0de941d680
map_version: '2'
---
# states::SuspectRecord

Describes a suspect record: its id, type, title, and the anchors whose code changed.

## Behaviour

- Built by suspects, one entry per record id
- several drifted anchors on the same record are appended to changed

## Terms

suspect, stale record, drifted note, changed anchors

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:32-37`
- **Signature:** `pub struct SuspectRecord`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
