---
id: runner-git
type: summary
status: unverified
title: runner::git
source: myrn summarize
about: sym:bench/src/runner.rs#runner::git
anchors:
- sym:bench/src/runner.rs#runner::git@d5428fb6083a3db1
---
# runner::git

Runs git with the given args in dir and returns trimmed stdout. A non-zero exit becomes an error that includes git's stderr.

## Connections

- **Location:** `bench/src/runner.rs:107-122`
- **Signature:** `fn git(dir: &Path, args: &[&str]) -> Result<String>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-prepare]], [[runner-validate]]
