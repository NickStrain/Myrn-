---
id: capture-to-entry
type: summary
status: unverified
title: capture::to_entry
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::to_entry
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::to_entry@990d47f4dadd3e50
---
# capture::to_entry

Converts an Event into an Entry for session_start, prompt, tool, or session_end, or returns None. For tool results, each listed file must be repo-relative and not ignored; if files were listed and none survived, it returns None even when a command is present, and it also returns None when files, command, and error are all empty. Commands matched by outcome::is_test_command get test set to pass or fail from ToolOutcome::Failure; symbols and commits are left to the caller.

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:25-98`
- **Signature:** `pub fn to_entry( agent: &str, event: &Event, root: &Path, ignore: &Gitignore, t: i64, ) -> Option<Entry>`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Calls:** [[capture-clean]], [[capture-is-ignored]], [[capture-relative]], [[outcome-is-test-command]], [[redact-redact]]
- **Called by:** [[engine-capture]]
- **Tests:** `paths_are_relative_and_private_paths_are_dropped`, `commands_are_redacted_and_hide_ignored_paths`, `test_runs_get_an_outcome`
