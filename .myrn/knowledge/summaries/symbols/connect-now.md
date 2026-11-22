---
id: connect-now
type: summary
status: unverified
title: connect::now
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::now
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::now@c46b3c90f30dd863
---
# connect::now

Returns the current Unix time in whole seconds as i64, or 0 if the system clock is before the epoch.

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:142-147`
- **Signature:** `pub fn now() -> i64`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Called by:** [[connect-run]], [[hook-handle]]
