---
id: capture-to-entry
type: summary
status: unverified
title: capture::to_entry
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::to_entry
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::to_entry@990d47f4dadd3e50
map_version: '2'
---
# capture::to_entry

Maps one Event to an episode Entry (session_start, prompt, tool, session_end), or returns None when nothing is worth keeping. The engine fills in symbols and commits afterwards.

## Behaviour

- ToolResult with files ["/r/src/a.rs", "src/./b.rs"] and cwd /r -> files ["src/a.rs", "src/b.rs"], duplicates removed
- ToolResult whose files are all ignored or outside the repo -> None, even when a command is present
- test command with Failure outcome -> test "fail"; other outcomes -> "pass"; non-test command -> no test field

## Terms

log entry, event record, episode line, session event, tool result, test pass fail, drop event, prompt text

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:25-98`
- **Signature:** `pub fn to_entry( agent: &str, event: &Event, root: &Path, ignore: &Gitignore, t: i64, ) -> Option<Entry>`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Calls:** [[capture-clean]], [[capture-is-ignored]], [[capture-relative]], [[outcome-is-test-command]], [[redact-redact]]
- **Called by:** [[engine-capture]]
- **Tests:** `paths_are_relative_and_private_paths_are_dropped`, `commands_are_redacted_and_hide_ignored_paths`, `test_runs_get_an_outcome`
