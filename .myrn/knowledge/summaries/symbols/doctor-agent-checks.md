---
id: doctor-agent-checks
type: summary
status: unverified
title: doctor::agent_checks
source: myrn summarize
about: sym:crates/myrn/src/cmd/doctor.rs#cmd::doctor::agent_checks
anchors:
- sym:crates/myrn/src/cmd/doctor.rs#cmd::doctor::agent_checks@e2acbb091d189a92
---
# doctor::agent_checks

If no agents are connected, returns one 'agents' Warn and stops early, so the recording, injection, and latency checks are skipped. Otherwise it emits an Ok line per agent (mode with '_' shown as '-', plus written files), one line per adapter inspect() finding (Fail if not ok), then always-Ok 'recording' and 'injection' checks and a 'hook latency' Warn when the median exceeds 500 ms (median is ms[len/2], so the upper value for even counts); agent names unknown to myrn_adapters::by_name are silently skipped, and state() or install_ctx errors propagate.

## Connections

- **Location:** `crates/myrn/src/cmd/doctor.rs:37-110`
- **Signature:** `fn agent_checks(engine: &Engine) -> Result<Vec<Check>>`
- **In:** [[file-crates-myrn-src-cmd-doctor-rs]]
- **Calls:** [[mode-parse]], [[by-name]], [[engine-root]], [[engine-state]], [[connect-install-ctx]]
- **Called by:** [[doctor-run]]
