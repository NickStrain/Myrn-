---
id: event-event
type: summary
status: unverified
title: event::Event
source: myrn summarize
about: sym:crates/myrn-model/src/event.rs#event::Event
anchors:
- sym:crates/myrn-model/src/event.rs#event::Event@f6988565aec5da79
map_version: '2'
---
# event::Event

Wraps one normalized event with its session id, optional working directory, and the EventKind payload under the key kind.

## Behaviour

- {"session_id":"s1","kind":{...}} -> kind holds the tagged EventKind object
- cwd missing or null in JSON -> cwd None
- cwd is documented as an absolute path reported by the agent, and is not validated here

## Terms

session id, working directory, cwd, envelope, normalized event, agent payload

## Connections

- **Location:** `crates/myrn-model/src/event.rs:7-12`
- **Signature:** `pub struct Event`
- **In:** [[file-crates-myrn-model-src-event-rs]]
