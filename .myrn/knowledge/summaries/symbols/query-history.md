---
id: query-history
type: summary
status: unverified
title: query::history
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::history
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::history@441a427b80837591
map_version: '2'
---
# query::history

Runs engine.history for a path with a limit and emits the result, rendered by myrn_mcp's history renderer.

## Behaviour

- path: &str, limit: usize -> engine.history(path, limit), rendered by myrn_mcp::tools::history::render
- Rendering is delegated to the MCP tool module, so text output matches the MCP tool
- Any error from ctx.engine() or engine.history is returned via ?

## Terms

file history, path log, past changes, changelog, change history, revisions

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:21-25`
- **Signature:** `pub fn history(ctx: &Ctx, path: &str, limit: usize) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-history]], [[history-render]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
