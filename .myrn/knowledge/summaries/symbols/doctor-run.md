---
id: doctor-run
type: summary
status: unverified
title: doctor::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/doctor.rs#cmd::doctor::run
anchors:
- sym:crates/myrn/src/cmd/doctor.rs#cmd::doctor::run@af9048eeef434b61
map_version: '2'
---
# doctor::run

Opens the engine at the current directory, runs engine.doctor(), replaces its "agents" row with agent_checks output, prints every row, and returns FAILURE if any row is Fail.

## Behaviour

- cwd is the engine root; ctx.data_dir is passed to Engine::open when set
- engine.doctor() rows named "agents" are dropped before agent_checks rows are appended
- any Level::Fail gives ExitCode::FAILURE; Warn alone still gives SUCCESS

## Terms

doctor, health check, diagnose, exit code, status report, engine diagnostics

## Connections

- **Location:** `crates/myrn/src/cmd/doctor.rs:11-35`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-doctor-rs]]
- **Calls:** [[engine-open]], [[engine-doctor]], [[doctor-agent-checks]], [[ctx-emit]]
- **Called by:** [[main-main]]
