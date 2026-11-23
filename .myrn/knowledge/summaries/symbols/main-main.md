---
id: main-main
type: summary
status: unverified
title: main
source: myrn summarize
about: sym:crates/myrn/src/main.rs#main
anchors:
- sym:crates/myrn/src/main.rs#main@797884da58737f67
map_version: '2'
---
# main

Parses the CLI, builds `cmd::Ctx` from the global flags, dispatches the subcommand to its handler, and maps the result to the process exit code.

## Behaviour

- `Ok(code)` from the handler -> that code is returned as the process exit status
- `Err(e)` -> prints `myrn: {e:#}` to stderr and exits with `ExitCode::FAILURE`
- `EmbedHelper` runs `myrn_embed::helper::serve` with `idle_secs` as a Duration and returns SUCCESS on `Ok`

## Terms

entry point, dispatch, exit status, error output, startup, router

## Connections

- **Location:** `crates/myrn/src/main.rs:238-327`
- **Signature:** `fn main() -> ExitCode`
- **In:** [[file-crates-myrn-src-main-rs]]
- **Calls:** [[helper-serve]], [[helper-serve-helper]], [[connect-run]], [[debug-review]], [[diff-run]], [[disconnect-run]], [[doctor-run]], [[embeddings-enable]], [[embeddings-disable]], [[embeddings-status]], [[forget-run]], [[gc-run]], [[hook-run]], [[init-run]], [[log-run]], [[mcp-run]], [[publish-run]], [[query-explain]], [[query-impact]], [[query-history]], [[query-context]], [[query-brief]], [[rebuild-run]], [[show-run]], [[status-run]]
