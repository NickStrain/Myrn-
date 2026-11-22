---
id: main-main
type: summary
status: unverified
title: main
source: myrn summarize
about: sym:crates/myrn/src/main.rs#main
anchors:
- sym:crates/myrn/src/main.rs#main@d01f21566c30f02d
---
# main

Parses Cli, builds cmd::Ctx from data_dir and json, and dispatches each Command to its cmd handler; Context joins the task words with spaces. Handler errors print as 'myrn: ' plus the error chain to stderr and return ExitCode::FAILURE, otherwise the handler's ExitCode is returned.

## Connections

- **Location:** `crates/myrn/src/main.rs:151-208`
- **Signature:** `fn main() -> ExitCode`
- **In:** [[file-crates-myrn-src-main-rs]]
- **Calls:** [[connect-run]], [[debug-review]], [[disconnect-run]], [[doctor-run]], [[hook-run]], [[init-run]], [[log-run]], [[mcp-run]], [[query-explain]], [[query-impact]], [[query-history]], [[query-context]], [[query-brief]], [[rebuild-run]], [[show-run]], [[status-run]], [[summarize-run]]
