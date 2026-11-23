---
id: summarize-options
type: summary
status: unverified
title: summarize::Options
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::Options
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::Options@f45a735df6a82afb
map_version: '2'
---
# summarize::Options

Settings for one Engine::summarize run: dry-run flag, optional file limit, parallel job count and the model command argv.

## Behaviour

- dry_run true -> counts only, no model call and no note write
- limit Some(n) -> first n stale file jobs run, the rest go to deferred_files; directories still run
- command [prog, args...] -> prog is run with args; empty command -> error '[summarize] command is empty'

## Terms

dry run, file limit, concurrency, model command, run settings, parallel jobs

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:47-54`
- **Signature:** `pub struct Options`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
