---
id: mcp-run
type: summary
status: unverified
title: mcp::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/mcp.rs#cmd::mcp::run
anchors:
- sym:crates/myrn/src/cmd/mcp.rs#cmd::mcp::run@0b8126644f6ac717
map_version: '2'
---
# mcp::run

Builds the engine from the command context, starts the MCP server on stdio with it, and returns success once serve returns.

## Behaviour

- ctx.engine() fails -> the error is returned via `?` and serve is never called
- myrn_mcp::serve(engine) returns Err -> the error propagates as Err and ExitCode::SUCCESS is not returned
- serve returns Ok -> returns Ok(ExitCode::SUCCESS)

## Terms

mcp subcommand, stdio server, model context protocol, start server, serve, entry point, exit code

## Connections

- **Location:** `crates/myrn/src/cmd/mcp.rs:9-13`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-mcp-rs]]
- **Calls:** [[server-serve]], [[ctx-engine]]
- **Called by:** [[main-main]]
