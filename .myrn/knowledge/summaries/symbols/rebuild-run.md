---
id: rebuild-run
type: summary
status: unverified
title: rebuild::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/rebuild.rs#cmd::rebuild::run
anchors:
- sym:crates/myrn/src/cmd/rebuild.rs#cmd::rebuild::run@51aa3a7f19a79216
---
# rebuild::run

Opens `Engine` at the current working directory, using `ctx.data_dir` as an optional override, then calls `engine.rebuild()` and emits the returned report. The text output reads 'Rebuilt the index: N files, N symbols, N commits, N notes in X.Xs', with counts formatted by `thousands` and time converted from `report.millis`. It returns an `ExitCode` and propagates errors from opening or rebuilding.

## Connections

- **Location:** `crates/myrn/src/cmd/rebuild.rs:10-24`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-rebuild-rs]]
- **Calls:** [[engine-open]], [[engine-rebuild]], [[ctx-emit]], [[cmd-thousands]]
- **Called by:** [[main-main]]
