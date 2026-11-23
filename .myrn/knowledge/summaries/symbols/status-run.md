---
id: status-run
type: summary
status: unverified
title: status::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/status.rs#cmd::status::run
anchors:
- sym:crates/myrn/src/cmd/status.rs#cmd::status::run@5366c84aa0298a9a
map_version: '2'
---
# status::run

Loads the engine and its status, then builds a multi-line text report of index, graph, link, record, note, conflict, and agent state. Returns the `ExitCode` from `ctx.emit`, with errors from engine setup or `status()` propagated via `?`.

## Behaviour

- project id is cut to its first 12 chars; if provisional_id is set, ' (temporary)' is appended
- syntax error count appears after files only when files_with_errors > 0; note flags (malformed, dangling links, duplicate ids) appear in parentheses with 'see myrn doctor' only when nonzero; missing edge kinds in s.edges read as 0
- no connected agents -> 'none connected'; empty conflicts -> conflicts line omitted; each conflict prints both ids, visibility, and titles, plus a hint to forget or supersede one

## Terms

health summary, index health, record counts, connected agents, conflicting records, syntax errors, dangling links, temporary project

## Connections

- **Location:** `crates/myrn/src/cmd/status.rs:9-92`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-status-rs]]
- **Calls:** [[engine-status]], [[ctx-engine]], [[ctx-emit]], [[cmd-thousands]]
- **Called by:** [[main-main]]
