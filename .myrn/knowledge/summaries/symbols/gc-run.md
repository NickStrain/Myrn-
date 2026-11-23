---
id: gc-run
type: summary
status: unverified
title: gc::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/gc.rs#cmd::gc::run
anchors:
- sym:crates/myrn/src/cmd/gc.rs#cmd::gc::run@856a3f0298a5cab0
map_version: '2'
---
# gc::run

Opens the engine, runs gc with the age cutoff and dry-run flag, and emits a report of removed duplicates, differing duplicates, dropped index rows, and compacted session logs. It adds a database size change only when not a dry run.

## Behaviour

- dry_run=true -> report uses 'would be removed/dropped/compacted' wording and omits the database MB line; the engine is still called with dry_run=true
- older_than=30 -> session logs older than 30 days are compacted; the value is in days and is echoed in the report
- duplicates whose content differs from the indexed copy are listed as 'left for you' and are not removed

## Terms

garbage collect, cleanup, dedupe, dry run, compact session logs, prune index, database size, report

## Connections

- **Location:** `crates/myrn/src/cmd/gc.rs:10-37`
- **Signature:** `pub fn run(ctx: &Ctx, older_than: u64, dry_run: bool) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-gc-rs]]
- **Calls:** [[engine-gc]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
