---
id: codex-translate
type: summary
status: unverified
title: Codex::translate
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::translate
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::translate@ba727f23bc990368
map_version: '2'
---
# Codex::translate

Maps a Codex hook payload to an Event. The payload's hook_event_name takes precedence over the event argument, and unknown events return None.

## Behaviour

- PostToolUse apply_patch -> ToolResult with files from the patch, command None, outcome FileEdit
- PostToolUse with non-zero exit -> outcome Failure, command taken from tool_input.command
- Stop -> SessionEnd with reason 'stop' and continued from stop_hook_active; unknown event -> Ok(None)

## Terms

event mapping, payload parsing, hook input, normalize, tool outcome

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:334-393`
- **Signature:** `fn translate(&self, event: &str, p: &Value) -> Result<Option<Event>>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-str-field]], [[codex-patch-files]], [[codex-failure-text]]
