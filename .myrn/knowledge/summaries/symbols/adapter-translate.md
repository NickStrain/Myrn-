---
id: adapter-translate
type: summary
status: unverified
title: Adapter::translate
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::translate
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::translate@95d2e854325b286e
map_version: '2'
---
# Adapter::translate

Converts an agent hook payload into a myrn Event, using the event name from the hook command line.

## Behaviour

- Event name not used by myrn -> Ok(None)
- Valid payload for a used event -> Ok(Some(Event))
- Payload that cannot be parsed -> Err

## Terms

hook payload, event mapping, parse hook, convert event, ignored event

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:82`
- **Signature:** `fn translate(&self, event: &str, payload: &Value) -> Result<Option<Event>>;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[hook-handle]]
