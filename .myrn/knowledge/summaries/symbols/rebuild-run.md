---
id: rebuild-run
type: summary
status: unverified
title: rebuild::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/rebuild.rs#cmd::rebuild::run
anchors:
- sym:crates/myrn/src/cmd/rebuild.rs#cmd::rebuild::run@51aa3a7f19a79216
map_version: '2'
---
# rebuild::run

Opens the engine at the current directory with the optional data dir, runs a full rebuild, and emits the report as a human-readable summary line.

## Behaviour

- current_dir() fails -> error returned before the engine is opened
- ctx.data_dir None -> Engine::open receives None; Some -> passed as a path
- report.millis 2500 -> summary ends with '2.5s' (millis / 1000, one decimal)

## Terms

reindex, recreate database, index summary, rebuild output, database regeneration, rebuild report, notes count

## Connections

- **Location:** `crates/myrn/src/cmd/rebuild.rs:10-24`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-rebuild-rs]]
- **Calls:** [[engine-open]], [[engine-rebuild]], [[ctx-emit]], [[cmd-thousands]]
- **Called by:** [[main-main]]
