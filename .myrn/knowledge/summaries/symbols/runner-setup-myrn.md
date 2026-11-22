---
id: runner-setup-myrn
type: summary
status: unverified
title: runner::setup_myrn
source: myrn summarize
about: sym:bench/src/runner.rs#runner::setup_myrn
anchors:
- sym:bench/src/runner.rs#runner::setup_myrn@35fa664519d33390
---
# runner::setup_myrn

Runs myrn init and then myrn connect --agent claude --yes in work with MYRN_HOME set. When inject is true, it changes 'inject = false' to 'inject = true' in .myrn/project.toml and errors if that line is missing.

## Connections

- **Location:** `bench/src/runner.rs:422-456`
- **Signature:** `fn setup_myrn(myrn: &Path, work: &Path, home: &Path, inject: bool) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-run-inner]]
