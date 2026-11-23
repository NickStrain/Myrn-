---
id: summarize-run
type: summary
status: unverified
title: summarize::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/summarize.rs#cmd::summarize::run
anchors:
- sym:crates/myrn/src/cmd/summarize.rs#cmd::summarize::run@9714a0b12f05593d
map_version: '2'
---
# summarize::run

Runs the summarize engine with the given flags, streams progress to stderr unless JSON output is on, then prints the report and returns exit code 1 if any file failed, otherwise 0.

## Behaviour

- dry_run true -> prints only counts of out of date files, functions and types, and directories, plus a hint to run again without it
- ctx.json true -> progress lines on stderr are suppressed; the report is still emitted
- report.failures non-empty -> ExitCode 1; empty -> ExitCode SUCCESS; a failure in the emit call is discarded by the let _ binding

## Terms

project map, dry run, stale, out of date, exit code, parallel workers, progress output, deferred files

## Connections

- **Location:** `crates/myrn/src/cmd/summarize.rs:11-86`
- **Signature:** `pub fn run( ctx: &Ctx, dry_run: bool, limit: Option<usize>, jobs: Option<usize>, ) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-summarize-rs]]
- **Calls:** [[engine-summarize]], [[ctx-engine]], [[ctx-emit]], [[cmd-thousands]]
- **Called by:** [[main-main]]
