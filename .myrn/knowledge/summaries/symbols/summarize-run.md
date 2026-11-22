---
id: summarize-run
type: summary
status: unverified
title: summarize::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/summarize.rs#cmd::summarize::run
anchors:
- sym:crates/myrn/src/cmd/summarize.rs#cmd::summarize::run@9714a0b12f05593d
---
# summarize::run

Takes ctx, dry_run, an optional limit, and optional jobs; jobs overrides config.summarize.jobs when given, and config.summarize.command is passed through. It calls engine.summarize, which writes notes to the vault's summaries directory (dry_run only reports stale counts), then emits the report to stdout. It returns exit code 1 if any file failed to summarize (the report is still printed) and 0 otherwise, and propagates engine or emit errors as Err.

## Connections

- **Location:** `crates/myrn/src/cmd/summarize.rs:11-86`
- **Signature:** `pub fn run( ctx: &Ctx, dry_run: bool, limit: Option<usize>, jobs: Option<usize>, ) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-summarize-rs]]
- **Calls:** [[engine-summarize]], [[ctx-engine]], [[ctx-emit]], [[cmd-thousands]]
- **Called by:** [[main-main]]
