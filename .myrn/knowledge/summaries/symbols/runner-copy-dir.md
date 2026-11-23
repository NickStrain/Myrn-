---
id: runner-copy-dir
type: summary
status: unverified
title: runner::copy_dir
source: myrn summarize
about: sym:bench/src/runner.rs#runner::copy_dir
anchors:
- sym:bench/src/runner.rs#runner::copy_dir@d962af671505ab8d
map_version: '2'
---
# runner::copy_dir

Recursively copies a directory tree, creating the destination and overwriting files that already exist.

## Behaviour

- Destination missing -> created, including parents
- Same file name at destination -> overwritten
- Symlinks are not recursed into; they go to std::fs::copy

## Terms

recursive copy, directory tree, duplicate, mirror, copy folder

## Connections

- **Location:** `bench/src/runner.rs:530-542`
- **Signature:** `fn copy_dir(from: &Path, to: &Path) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-copy-dir]]
- **Called by:** [[runner-setup-myrn]], [[runner-copy-dir]], [[runner-build-map]]
