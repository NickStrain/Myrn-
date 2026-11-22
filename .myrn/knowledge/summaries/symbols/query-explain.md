---
id: query-explain
type: summary
status: unverified
title: query::explain
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::explain
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::explain@d8da83845dc0c1ed
---
# query::explain

Calls engine.explain(symbol) and emits the result, with text rendered through the result's render method using each entry's own render. Errors from engine creation or the query propagate as anyhow errors.

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:9-13`
- **Signature:** `pub fn explain(ctx: &Ctx, symbol: &str) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-explain]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
