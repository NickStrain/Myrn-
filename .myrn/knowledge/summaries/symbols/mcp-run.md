---
id: mcp-run
type: summary
status: unverified
title: mcp::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/mcp.rs#cmd::mcp::run
anchors:
- sym:crates/myrn/src/cmd/mcp.rs#cmd::mcp::run@0b8126644f6ac717
---
# mcp::run

Takes `&Ctx`, calls `ctx.engine()?` to build the engine, then blocks in `myrn_mcp::serve(engine)` for the stdio session; any error from engine construction or serving propagates via `?`, and it returns `ExitCode::SUCCESS` only after `serve` returns Ok. No side effects beyond what `ctx.engine()` and `serve` perform.

## Connections

- **Location:** `crates/myrn/src/cmd/mcp.rs:9-13`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-mcp-rs]]
- **Calls:** [[server-serve]], [[ctx-engine]]
- **Called by:** [[main-main]]
