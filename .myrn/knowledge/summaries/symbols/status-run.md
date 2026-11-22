---
id: status-run
type: summary
status: unverified
title: status::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/status.rs#cmd::status::run
anchors:
- sym:crates/myrn/src/cmd/status.rs#cmd::status::run@3636bfa2f120dad3
---
# status::run

Takes `&Ctx`, calls `ctx.engine()?.status()?`, and emits the status via `ctx.emit`; the text renderer shows the first 12 chars of project_id (marked "(temporary)" when provisional_id is set), appends a syntax-error count only when files_with_errors > 0, and appends note problem flags (malformed, dangling links, duplicate ids) plus a `myrn doctor` pointer only when any are nonzero. Returns `Result<ExitCode>`, and it has no side effects beyond reading status and writing output.

## Connections

- **Location:** `crates/myrn/src/cmd/status.rs:9-79`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-status-rs]]
- **Calls:** [[engine-status]], [[ctx-engine]], [[ctx-emit]], [[cmd-thousands]]
- **Called by:** [[main-main]]
