---
id: claudecode-translate
type: summary
status: unverified
title: ClaudeCode::translate
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::translate
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::translate@34c1f74bd6f43016
map_version: '2'
---
# ClaudeCode::translate

Converts a Claude Code hook payload into a myrn Event. The event name comes from hook_event_name, falling back to the event argument. Returns None for events it does not handle.

## Behaviour

- SessionStart -> SessionStart with source; UserPromptSubmit -> prompt text
- PostToolUse or PostToolUseFailure -> ToolResult; a failure is always Failure, and Edit, Write, MultiEdit or NotebookEdit are FileEdit
- Stop -> SessionEnd with reason stop and continued from stop_hook_active; Notification or any other event -> None

## Terms

payload parsing, hook event, normalize, map event, decode

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:255-306`
- **Signature:** `fn translate(&self, event: &str, p: &Value) -> Result<Option<Event>>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-str-field]]
