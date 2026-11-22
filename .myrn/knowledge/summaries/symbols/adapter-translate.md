---
id: adapter-translate
type: summary
status: unverified
title: Adapter::translate
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::translate
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::translate@95d2e854325b286e
---
# Adapter::translate

Converts an agent hook payload for the given event name into a myrn Event, or returns None for events myrn does not use.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:82`
- **Signature:** `fn translate(&self, event: &str, payload: &Value) -> Result<Option<Event>>;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[hook-handle]]
