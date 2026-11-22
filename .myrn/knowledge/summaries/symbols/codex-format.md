---
id: codex-format
type: summary
status: unverified
title: Codex::format
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::format
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::format@04754051f4ce017e
---
# Codex::format

Produces the hook output JSON for a given event. Stop returns a block decision with the context as the reason, which makes the agent take one more step. SessionStart, UserPromptSubmit, and PostToolUse return hookSpecificOutput with additionalContext. Empty context or any other event returns an empty string, and the context is clipped.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:395-408`
- **Signature:** `fn format(&self, event: &str, context: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-clip]]
