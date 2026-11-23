---
id: event-eventkind
type: summary
status: unverified
title: event::EventKind
source: myrn summarize
about: sym:crates/myrn-model/src/event.rs#event::EventKind
anchors:
- sym:crates/myrn-model/src/event.rs#event::EventKind@b263e38d2ca70519
map_version: '2'
---
# event::EventKind

Tagged enum of the four event types an adapter can emit: session start, prompt submitted, tool result, and session end. The serde tag is the field named event, with snake_case variant names.

## Behaviour

- {"event":"session_start","source":"resume"} -> SessionStart with source Some("resume")
- {"event":"session_end"} with no continued key -> SessionEnd with continued false
- SessionEnd continued true means the agent is already continuing because a stop hook asked it to, so the caller must not ask again

## Terms

event kind, event type, hook event, session start, session end, prompt submitted, stop hook, tool result event

## Connections

- **Location:** `crates/myrn-model/src/event.rs:16-33`
- **Signature:** `pub enum EventKind`
- **In:** [[file-crates-myrn-model-src-event-rs]]
