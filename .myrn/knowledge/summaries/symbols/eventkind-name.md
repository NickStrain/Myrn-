---
id: eventkind-name
type: summary
status: unverified
title: EventKind::name
source: myrn summarize
about: sym:crates/myrn-model/src/event.rs#event::EventKind::name
anchors:
- sym:crates/myrn-model/src/event.rs#event::EventKind::name@1a95f137aa4f1a15
map_version: '2'
---
# EventKind::name

Returns the fixed snake_case label for the variant, matching the serde tag string.

## Behaviour

- SessionStart -> "session_start"
- ToolResult(_) -> "tool_result" regardless of tool or outcome
- SessionEnd -> "session_end" regardless of reason or continued

## Terms

event name, variant name, label, tag string, event type string

## Connections

- **Location:** `crates/myrn-model/src/event.rs:56-63`
- **Signature:** `pub fn name(&self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-event-rs]]
