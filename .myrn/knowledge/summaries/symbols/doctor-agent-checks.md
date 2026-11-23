---
id: doctor-agent-checks
type: summary
status: unverified
title: doctor::agent_checks
source: myrn summarize
about: sym:crates/myrn/src/cmd/doctor.rs#cmd::doctor::agent_checks
anchors:
- sym:crates/myrn/src/cmd/doctor.rs#cmd::doctor::agent_checks@e2acbb091d189a92
map_version: '2'
---
# doctor::agent_checks

Builds the check rows from engine state: a Warn "agents" row when no agent is connected, otherwise a connected row and adapter inspection rows per agent, plus recording consent, injection setting and hook latency rows.

## Behaviour

- no agents in state -> single Warn row "agents" saying run `myrn connect`, then returns early with no other rows
- agent name with no matching adapter (by_name returns None) is skipped silently
- hook latency: median is sorted ms[len/2] (upper median for even counts); Warn if above 500 ms; zero calls gives Ok "no hook calls yet"

## Terms

agent connection, install check, adapter inspection, hook latency, median latency, recording consent, injection setting, connect

## Connections

- **Location:** `crates/myrn/src/cmd/doctor.rs:37-110`
- **Signature:** `fn agent_checks(engine: &Engine) -> Result<Vec<Check>>`
- **In:** [[file-crates-myrn-src-cmd-doctor-rs]]
- **Calls:** [[mode-parse]], [[by-name]], [[engine-root]], [[engine-state]], [[connect-install-ctx]]
- **Called by:** [[doctor-run]]
