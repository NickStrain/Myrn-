---
id: codex-translate
type: summary
status: unverified
title: Codex::translate
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::translate
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::translate@ba727f23bc990368
---
# Codex::translate

Maps a Codex hook payload to an Event for SessionStart, UserPromptSubmit, PostToolUse, Stop, and SessionEnd, and returns None for other names. For PostToolUse it extracts files from apply_patch or from file_path and path inputs, and it drops the command for apply_patch. For Stop it sets continued from stop_hook_active.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:334-393`
- **Signature:** `fn translate(&self, event: &str, p: &Value) -> Result<Option<Event>>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-str-field]], [[codex-patch-files]], [[codex-failure-text]]
