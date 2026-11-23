---
id: runner-git
type: summary
status: unverified
title: runner::git
source: myrn summarize
about: sym:bench/src/runner.rs#runner::git
anchors:
- sym:bench/src/runner.rs#runner::git@d5428fb6083a3db1
map_version: '2'
---
# runner::git

Runs git with the given arguments in a directory and returns its trimmed stdout.

## Behaviour

- Non-zero exit -> error with the args and trimmed stderr
- Stdin is null; spawn failure gets the context "running git ..."
- Trailing newline and surrounding whitespace are removed from the output

## Terms

shell out, git command, subprocess, git wrapper, run git, trim output

## Connections

- **Location:** `bench/src/runner.rs:125-140`
- **Signature:** `fn git(dir: &Path, args: &[&str]) -> Result<String>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-prepare]], [[runner-build-map]], [[runner-validate]]
