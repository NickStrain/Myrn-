---
id: claudecode-format
type: summary
status: unverified
title: ClaudeCode::format
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::format
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::format@2a941b534f777dca
map_version: '2'
---
# ClaudeCode::format

Builds the JSON a hook prints to feed context back to Claude Code. Returns an empty string for blank context or for events that take no output.

## Behaviour

- Stop with text -> decision block with the clipped text as reason, so the agent takes one more step
- SessionStart, UserPromptSubmit, PostToolUse, PostToolUseFailure -> hookSpecificOutput with clipped additionalContext
- Blank or whitespace context, or SessionEnd -> empty string

## Terms

output json, additional context, hook response, block decision, clip, stdout

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:308-321`
- **Signature:** `fn format(&self, event: &str, context: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-clip]]
