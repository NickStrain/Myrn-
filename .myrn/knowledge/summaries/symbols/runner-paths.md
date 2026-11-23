---
id: runner-paths
type: summary
status: unverified
title: runner::Paths
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Paths
anchors:
- sym:bench/src/runner.rs#runner::Paths@03a3d686b78528c5
map_version: '2'
---
# runner::Paths

Directories used by the runner: the repository checkouts, a scratch work directory, and a shared cargo target directory.

## Behaviour

- work holds each run's clone, and the myrn home too when shared_store is off
- target is shared across runs of the same repo so builds are reused
- checkouts must already contain .git, or run_inner fails

## Terms

directories, folders, scratch space, build cache, target dir, checkout location, paths

## Connections

- **Location:** `bench/src/runner.rs:115-121`
- **Signature:** `pub struct Paths`
- **In:** [[file-bench-src-runner-rs]]
