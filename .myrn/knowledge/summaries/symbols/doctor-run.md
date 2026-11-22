---
id: doctor-run
type: summary
status: unverified
title: doctor::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/doctor.rs#cmd::doctor::run
anchors:
- sym:crates/myrn/src/cmd/doctor.rs#cmd::doctor::run@af9048eeef434b61
---
# doctor::run

Opens the Engine at the current directory with ctx.data_dir, takes engine.doctor() checks, drops any check named 'agents', and appends agent_checks; it emits the list via ctx.emit with a renderer that prints ok/warn/FAIL and pads the name to 15 columns, then returns ExitCode::FAILURE only if some check is Level::Fail (Warn does not fail). Errors from opening the engine, agent_checks, or emit propagate via ?.

## Connections

- **Location:** `crates/myrn/src/cmd/doctor.rs:11-35`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-doctor-rs]]
- **Calls:** [[engine-open]], [[engine-doctor]], [[doctor-agent-checks]], [[ctx-emit]]
- **Called by:** [[main-main]]
