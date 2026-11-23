---
id: runner-prepare
type: summary
status: unverified
title: runner::prepare
source: myrn summarize
about: sym:bench/src/runner.rs#runner::prepare
anchors:
- sym:bench/src/runner.rs#runner::prepare@404cef6ddd1ad851
map_version: '2'
---
# runner::prepare

Recreates the work directory as a clone of checkout at repo.commit on branch bench, strips history that leads back to the unbroken code, and folds the task setup patch into HEAD as an amended commit.

## Behaviour

- An existing work directory is deleted first
- With task.setup: applies the patch, stages everything, and amends the commit while keeping its committer date
- Removes the remote, other branches, tags, and reflogs, then runs gc --prune=now to drop unreachable objects

## Terms

reset workspace, fresh clone, setup commit, scrub history, hide history, sanitize, leak prevention, clean checkout

## Connections

- **Location:** `bench/src/runner.rs:167-220`
- **Signature:** `pub fn prepare(checkout: &Path, repo: &Repo, task: &Task, work: &Path) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-git]], [[runner-git-apply]]
- **Called by:** [[retrieval-one]], [[runner-run-inner]], [[runner-validate]]
