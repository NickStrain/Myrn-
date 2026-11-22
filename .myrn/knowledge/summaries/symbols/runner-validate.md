---
id: runner-validate
type: summary
status: unverified
title: runner::validate
source: myrn summarize
about: sym:bench/src/runner.rs#runner::validate
anchors:
- sym:bench/src/runner.rs#runner::validate@03d1f1bcc064b89d
---
# runner::validate

Checks a task without an agent. For non-Question tasks it prepares the workspace, requires a clean git status, expects the check to fail, applies the solution (or reverse-applies the setup when there is no solution), and expects the check to pass. Question tasks only report their required answer phrases, and any error is stored in note instead of being returned.

## Connections

- **Location:** `bench/src/runner.rs:564-606`
- **Signature:** `pub fn validate(task: &Task, repo: &Repo, paths: &Paths) -> Validation`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-git]], [[runner-git-apply]], [[runner-prepare]], [[runner-run-check]], [[runner-tail]]
- **Called by:** [[real-main]]
