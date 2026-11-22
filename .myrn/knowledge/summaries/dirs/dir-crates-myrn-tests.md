---
id: dir-crates-myrn-tests
type: summary
status: unverified
title: crates/myrn/tests/
source: myrn summarize
about: module:crates/myrn/tests
anchors:
- module:crates/myrn/tests@4fa8b8c35f4c9efc
---
# crates/myrn/tests/

Integration tests that drive the real `myrn` binary end to end, as an agent would, with MYRN_HOME set to a temp dir and a temp git repo built by the `setup`, `Env`, `hook`, `settings`, and `text` helpers. The single test file `connect.rs` covers init, `connect --agent` consent gating, Claude and Codex hooks (SessionStart, UserPromptSubmit, PostToolUse, Stop) with injection and per-session dedupe, `doctor` drift detection, `--pull-only` Codex setup, `disconnect`, and episode log checks for secret redaction, ignore paths, and `show episode:` graph output.

## Connections

- **In:** [[dir-crates-myrn]]
- **Files:** [[file-crates-myrn-tests-connect-rs]]
