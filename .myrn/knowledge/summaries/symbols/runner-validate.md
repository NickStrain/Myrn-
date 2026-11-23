---
id: runner-validate
type: summary
status: unverified
title: runner::validate
source: myrn summarize
about: sym:bench/src/runner.rs#runner::validate
anchors:
- sym:bench/src/runner.rs#runner::validate@03d1f1bcc064b89d
map_version: '2'
---
# runner::validate

Checks a task without an agent: prepares the workspace, confirms it is clean, runs the check on broken code, applies the solution, and runs the check again. Question tasks are skipped.

## Behaviour

- Question kind -> only note is set; both flags stay None
- Dirty status after prepare -> error note, and no check runs
- Solution applied forward if present; otherwise the setup patch is reversed; with neither -> error "no solution to apply"

## Terms

dry run, verify task, reference fix, gold patch, reverse setup, task check, sanity check run

## Connections

- **Location:** `bench/src/runner.rs:719-761`
- **Signature:** `pub fn validate(task: &Task, repo: &Repo, paths: &Paths) -> Validation`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-git]], [[runner-git-apply]], [[runner-prepare]], [[runner-run-check]], [[runner-tail]]
- **Called by:** [[real-main]]
