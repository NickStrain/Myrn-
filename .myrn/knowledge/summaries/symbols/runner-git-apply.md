---
id: runner-git-apply
type: summary
status: unverified
title: runner::git_apply
source: myrn summarize
about: sym:bench/src/runner.rs#runner::git_apply
anchors:
- sym:bench/src/runner.rs#runner::git_apply@5e94d9ae46f8f854
---
# runner::git_apply

Pipes a patch into git apply in dir, adding -R when reverse is true. Returns an error with git's stderr if the patch does not apply.

## Connections

- **Location:** `bench/src/runner.rs:124-144`
- **Signature:** `fn git_apply(dir: &Path, patch: &str, reverse: bool) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-prepare]], [[runner-validate]]
