---
id: log-run
type: summary
status: unverified
title: log::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/log.rs#cmd::log::run
anchors:
- sym:crates/myrn/src/cmd/log.rs#cmd::log::run@6cde0f1bd83ed3a6
---
# log::run

Without a session id, lists up to `limit` recent episodes as two-line rows (UTC time, agent, session, outcome, first task line, summary), or explains that nothing is recorded, depending on `capture_consent`. With a session id, emits that episode as JSON or text via render_one, and bails with an error if no session matches.

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs:12-61`
- **Signature:** `pub fn run(ctx: &Ctx, session: Option<&str>, limit: usize) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-log-rs]]
- **Calls:** [[engine-state]], [[engine-episodes]], [[engine-episode-log]], [[episode-outcome]], [[episode-summary]], [[episode-clip]], [[log-render-one]], [[log-datetime]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
