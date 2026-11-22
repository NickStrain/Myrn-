---
id: init-run
type: summary
status: unverified
title: init::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/init.rs#cmd::init::run
anchors:
- sym:crates/myrn/src/cmd/init.rs#cmd::init::run@bef1010607eb9db1
---
# init::run

Takes the `Ctx` (uses its `data_dir` override) and indexes the current directory via `Engine::init`, which creates the local store and returns an `Engine` plus a report of file, symbol, commit, and timing counts, with optional call, import, and test link counts. It emits that report through `ctx.emit`, noting a temporary id when there are no commits yet and the count of files with syntax errors, and it prints the shared config and private store paths. The engine status lookup uses `.ok()`, so a failed status call silently drops the syntax-error line rather than failing the command.

## Connections

- **Location:** `crates/myrn/src/cmd/init.rs:10-48`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-init-rs]]
- **Calls:** [[engine-init]], [[engine-root]], [[engine-status]], [[ctx-emit]], [[cmd-thousands]]
- **Called by:** [[main-main]]
