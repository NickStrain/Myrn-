---
id: query-brief
type: summary
status: unverified
title: query::brief
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::brief
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::brief@9faef4a03b91e777
---
# query::brief

Calls engine.brief(budget) with an optional token budget and emits the result. Text output is the brief text followed by its token count.

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:39-43`
- **Signature:** `pub fn brief(ctx: &Ctx, budget: Option<usize>) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-brief]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
