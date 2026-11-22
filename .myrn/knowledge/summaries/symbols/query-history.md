---
id: query-history
type: summary
status: unverified
title: query::history
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::history
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::history@441a427b80837591
---
# query::history

Calls engine.history(path, limit) and emits the result, with text rendered by myrn_mcp::tools::history::render. The path is a file path string and limit is a usize.

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:21-25`
- **Signature:** `pub fn history(ctx: &Ctx, path: &str, limit: usize) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-history]], [[history-render]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
