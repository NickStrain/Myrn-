---
id: eventkind-name
type: summary
status: unverified
title: EventKind::name
source: myrn summarize
about: sym:crates/myrn-model/src/event.rs#event::EventKind::name
anchors:
- sym:crates/myrn-model/src/event.rs#event::EventKind::name@1a95f137aa4f1a15
---
# EventKind::name

Returns the static snake_case label for the variant (session_start, prompt_submitted, tool_result, or session_end) without inspecting the payload or allocating.

## Connections

- **Location:** `crates/myrn-model/src/event.rs:56-63`
- **Signature:** `pub fn name(&self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-event-rs]]
