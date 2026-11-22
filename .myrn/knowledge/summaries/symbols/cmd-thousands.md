---
id: cmd-thousands
type: summary
status: unverified
title: cmd::thousands
source: myrn summarize
about: sym:crates/myrn/src/cmd/mod.rs#cmd::thousands
anchors:
- sym:crates/myrn/src/cmd/mod.rs#cmd::thousands@7bb6abacc98774e6
---
# cmd::thousands

Formats an i64 with comma separators every three digits and keeps a leading minus for negatives. n.abs() panics on i64::MIN in debug builds and wraps in release builds.

## Connections

- **Location:** `crates/myrn/src/cmd/mod.rs:57-71`
- **Signature:** `pub fn thousands(n: i64) -> String`
- **In:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Called by:** [[init-run]], [[rebuild-run]], [[status-run]], [[summarize-run]]
