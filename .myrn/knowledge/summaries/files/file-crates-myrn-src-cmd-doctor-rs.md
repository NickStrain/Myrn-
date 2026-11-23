---
id: file-crates-myrn-src-cmd-doctor-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/doctor.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/doctor.rs
anchors:
- file:crates/myrn/src/cmd/doctor.rs@55908316f0effe26
---
# crates/myrn/src/cmd/doctor.rs

myrn `doctor` subcommand. `run` is the entry point: it merges the engine's diagnostics with per-agent checks, prints them, and sets the exit code. `agent_checks` builds the agent, recording, injection and hook-latency rows from the engine state.

## Connections

- **Location:** `crates/myrn/src/cmd/doctor.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[doctor-run]], [[doctor-agent-checks]]
- **Imports:** [[file-crates-myrn-adapters-src-lib-rs]], [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
