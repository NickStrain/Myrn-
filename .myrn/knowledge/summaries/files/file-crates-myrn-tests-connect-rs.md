---
id: file-crates-myrn-tests-connect-rs
type: summary
status: unverified
title: crates/myrn/tests/connect.rs
source: myrn summarize
about: file:crates/myrn/tests/connect.rs
anchors:
- file:crates/myrn/tests/connect.rs@88847bae83f57d3a
---
# crates/myrn/tests/connect.rs

Integration test driving the real `myrn` binary end to end as an agent would: init, `connect --agent` consent gating, Claude/Codex hooks (SessionStart, UserPromptSubmit, PostToolUse, Stop) with injection and per-session dedupe, `doctor` drift detection, `--pull-only` Codex setup, `disconnect`, and the episode log with secret redaction, ignore paths, and `show episode:` graph checks. Helpers `setup`, `Env`, `hook`, `settings`, and `text` build a temp git repo and run the binary with MYRN_HOME pointed at a temp dir.

## Connections

- **Location:** `crates/myrn/tests/connect.rs`
- **In:** [[dir-crates-myrn-tests]]
- **Defines:** [[env]], [[setup-connect]], [[env-cmd]], [[env-myrn]], [[env-hook]], [[env-settings]], [[text]]
