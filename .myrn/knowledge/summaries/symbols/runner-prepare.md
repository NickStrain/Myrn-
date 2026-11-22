---
id: runner-prepare
type: summary
status: unverified
title: runner::prepare
source: myrn summarize
about: sym:bench/src/runner.rs#runner::prepare
anchors:
- sym:bench/src/runner.rs#runner::prepare@404cef6ddd1ad851
---
# runner::prepare

Deletes work if it exists and rebuilds it as a fresh clone of checkout at repo.commit on branch bench. It removes the origin remote, other branches and tags, and reflogs, runs gc --prune=now, and folds the task setup patch into HEAD by amending the commit with its original committer date.

## Connections

- **Location:** `bench/src/runner.rs:149-202`
- **Signature:** `pub fn prepare(checkout: &Path, repo: &Repo, task: &Task, work: &Path) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-git]], [[runner-git-apply]]
- **Called by:** [[runner-run-inner]], [[runner-validate]]
