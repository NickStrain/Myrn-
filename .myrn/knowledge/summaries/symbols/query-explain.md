---
id: query-explain
type: summary
status: unverified
title: query::explain
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::explain
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::explain@d8da83845dc0c1ed
map_version: '2'
---
# query::explain

Runs engine.explain on a symbol name and emits the result, rendered with the explain renderer.

## Behaviour

- symbol: &str -> engine.explain(symbol), then ctx.emit with r.render(|e| e.render())
- Any error from ctx.engine() or engine.explain is returned via ? before emitting
- Returns Result<ExitCode>

## Terms

explanation, describe symbol, symbol details, lookup, what is, symbol summary

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:9-13`
- **Signature:** `pub fn explain(ctx: &Ctx, symbol: &str) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-explain]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
