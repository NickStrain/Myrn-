---
id: log-run
type: summary
status: unverified
title: log::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/log.rs#cmd::log::run
anchors:
- sym:crates/myrn/src/cmd/log.rs#cmd::log::run@160cf7fbdc9e9025
map_version: '2'
---
# log::run

Lists sessions and records merged newest first, filtered by type and status, or prints one session when a session id is given. Emits the list as JSON or as a text table through ctx.emit.

## Behaviour

- session id with no episode but trace(id) succeeding -> bails pointing to 'myrn show' or 'myrn trace' because the id is a record
- session id matching nothing -> bails 'no recorded session matches'; other filters are ignored when a session id is given
- status set, or ntype other than episode/session -> sessions are skipped; ntype 'root-cause' is rewritten to 'root_cause' before querying

## Terms

history, timeline, episodes, recorded sessions, learned records, type filter, status filter, session detail

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs:14-131`
- **Signature:** `pub fn run( ctx: &Ctx, session: Option<&str>, limit: usize, ntype: Option<&str>, status: Option<&str>, map: bool, ) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-log-rs]]
- **Calls:** [[engine-state]], [[engine-episodes]], [[engine-episode-log]], [[episode-outcome]], [[episode-summary]], [[episode-clip]], [[engine-records]], [[engine-trace]], [[log-short-type]], [[log-render-one]], [[log-datetime]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
