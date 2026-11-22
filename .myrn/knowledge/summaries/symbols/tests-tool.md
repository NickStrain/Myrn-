---
id: tests-tool
type: summary
status: unverified
title: tests::tool
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::tests::tool
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::tests::tool@44e57b4144e8ae48
---
# tests::tool

Test helper that builds an Event of kind ToolResult with session 's', cwd '/r', tool 'Bash', the given outcome, files and command, and no error.

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:189-201`
- **Signature:** `fn tool(outcome: ToolOutcome, files: &[&str], command: Option<&str>) -> Event`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Tests:** `paths_are_relative_and_private_paths_are_dropped`, `commands_are_redacted_and_hide_ignored_paths`, `test_runs_get_an_outcome`
