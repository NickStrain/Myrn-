---
id: query-impact
type: summary
status: unverified
title: query::impact
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::impact
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::impact@5b12190a32133bd5
---
# query::impact

Calls engine.impact(symbol) and emits the result, rendering text with a limit of 25 items per impact render call. Errors propagate with ?.

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:15-19`
- **Signature:** `pub fn impact(ctx: &Ctx, symbol: &str) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-impact]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
