---
id: codex-format
type: summary
status: unverified
title: Codex::format
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::format
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::format@04754051f4ce017e
map_version: '2'
---
# Codex::format

Builds the stdout JSON for a hook from its context: a block decision with reason for Stop, or hookSpecificOutput additionalContext for SessionStart, UserPromptSubmit and PostToolUse.

## Behaviour

- blank context -> empty string
- Stop with context -> a decision of block with the clipped context as reason
- other events such as SessionEnd -> empty string

## Terms

hook output, stdout json, additional context, block decision, hook reply

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:395-408`
- **Signature:** `fn format(&self, event: &str, context: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-clip]]
