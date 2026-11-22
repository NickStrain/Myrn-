---
id: claudecode-format
type: summary
status: unverified
title: ClaudeCode::format
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::format
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::format@2a941b534f777dca
---
# ClaudeCode::format

Builds the hook stdout JSON for an event. For Stop it returns a block decision whose reason is the clipped context, which keeps the agent going one more step. For SessionStart, UserPromptSubmit, PostToolUse, and PostToolUseFailure it returns hookSpecificOutput.additionalContext. It returns an empty string for empty context or any other event.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:308-321`
- **Signature:** `fn format(&self, event: &str, context: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-clip]]
