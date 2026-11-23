---
id: query-impact
type: summary
status: unverified
title: query::impact
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::impact
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::impact@5b12190a32133bd5
map_version: '2'
---
# query::impact

Runs engine.impact on a symbol name and emits the result, rendered with the impact renderer capped at 25 items.

## Behaviour

- symbol: &str -> engine.impact(symbol), then r.render(|i| i.render(25))
- The render limit 25 is hard-coded and not a parameter
- Any error from ctx.engine() or engine.impact is returned via ? before emitting

## Terms

blast radius, callers, dependents, what breaks, change effect, affected code

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:15-19`
- **Signature:** `pub fn impact(ctx: &Ctx, symbol: &str) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-impact]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
