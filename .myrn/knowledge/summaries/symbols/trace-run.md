---
id: trace-run
type: summary
status: unverified
title: trace::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/trace.rs#cmd::trace::run
anchors:
- sym:crates/myrn/src/cmd/trace.rs#cmd::trace::run@6cb6dda1e6ee9ed5
map_version: '2'
---
# trace::run

Fetches the trace for a record id from the engine and renders it as a text report with header, status, file, provenance, commits, tests, anchors, links, note history, and body. Returns the ExitCode from `ctx.emit`.

## Behaviour

- session present -> prints a session block (id, agent, start time, first task line clipped to 90 chars, outcome) and ignores the record's source field
- no session and a source string -> prints 'source <s> (session log not found)'; no session and no source -> prints 'written by hand'
- status differs from written_status -> prints 'status X (written as Y)'; a commit sha shorter than 12 chars panics on the `[..12]` slice

## Terms

trace, provenance, origin, lineage, blame, history, session log, anchor

## Connections

- **Location:** `crates/myrn/src/cmd/trace.rs:12-91`
- **Signature:** `pub fn run(ctx: &Ctx, id: &str) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-trace-rs]]
- **Calls:** [[episode-outcome]], [[episode-summary]], [[episode-clip]], [[engine-trace]], [[log-datetime]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
