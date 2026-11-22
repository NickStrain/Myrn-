---
id: event-eventkind
type: summary
status: unverified
title: event::EventKind
source: myrn summarize
about: sym:crates/myrn-model/src/event.rs#event::EventKind
anchors:
- sym:crates/myrn-model/src/event.rs#event::EventKind@b263e38d2ca70519
---
# event::EventKind

Enum serialized with an internal tag field named event in snake_case. Variants: SessionStart{source}, PromptSubmitted{prompt}, ToolResult(ToolResult), and SessionEnd{reason, continued}. The continued flag defaults to false when absent and means the agent is already continuing because a stop hook asked it to, so the engine must not ask again or it will loop.

## Connections

- **Location:** `crates/myrn-model/src/event.rs:16-33`
- **Signature:** `pub enum EventKind`
- **In:** [[file-crates-myrn-model-src-event-rs]]
