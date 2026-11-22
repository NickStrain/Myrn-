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

Implements the `myrn doctor` subcommand. It combines engine diagnostics with per-agent adapter checks, prints them as a text table, and sets the exit code from whether any check failed. Its two functions are run (entry point, rendering, exit code) and agent_checks (agent, recording, injection, and hook-latency checks).

## Connections

- **Location:** `crates/myrn/src/cmd/doctor.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[doctor-run]], [[doctor-agent-checks]]
- **Imports:** [[file-crates-myrn-adapters-src-lib-rs]], [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
