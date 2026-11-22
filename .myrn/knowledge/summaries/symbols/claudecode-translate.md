---
id: claudecode-translate
type: summary
status: unverified
title: ClaudeCode::translate
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::translate
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::translate@34c1f74bd6f43016
---
# ClaudeCode::translate

Maps a Claude hook payload to an Event. SessionStart and UserPromptSubmit become their own kinds. PostToolUse and PostToolUseFailure become ToolResult, with files taken from file_path and notebook_path. Stop and SessionEnd become SessionEnd, and Stop sets continued from stop_hook_active. Other event names return None. A failure outcome takes precedence over the edit-tool check, which uses EDIT_TOOLS (Edit, Write, MultiEdit, NotebookEdit).

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:255-306`
- **Signature:** `fn translate(&self, event: &str, p: &Value) -> Result<Option<Event>>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-str-field]]
