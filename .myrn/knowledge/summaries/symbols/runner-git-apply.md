---
id: runner-git-apply
type: summary
status: unverified
title: runner::git_apply
source: myrn summarize
about: sym:bench/src/runner.rs#runner::git_apply
anchors:
- sym:bench/src/runner.rs#runner::git_apply@5e94d9ae46f8f854
map_version: '2'
---
# runner::git_apply

Feeds a patch to git apply on stdin in a directory, optionally applying it in reverse.

## Behaviour

- reverse true -> adds -R, which undoes the patch
- --whitespace=nowarn, so whitespace warnings do not fail the apply
- Non-zero exit -> error with git's stderr

## Terms

patch, diff, unapply, revert, reverse apply, apply diff

## Connections

- **Location:** `bench/src/runner.rs:142-162`
- **Signature:** `fn git_apply(dir: &Path, patch: &str, reverse: bool) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-prepare]], [[runner-validate]]
