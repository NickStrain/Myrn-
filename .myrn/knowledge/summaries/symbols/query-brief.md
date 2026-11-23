---
id: query-brief
type: summary
status: unverified
title: query::brief
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::brief
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::brief@9faef4a03b91e777
map_version: '2'
---
# query::brief

Builds a codebase brief under an optional budget and emits its text with its token count.

## Behaviour

- budget: None -> engine.brief(None), default size
- budget: Some(n) -> engine.brief(Some(n)), limited to n tokens by the engine
- Output is p.text followed by '[N tokens]'

## Terms

summary, overview, codebase summary, token budget, digest, project brief

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:39-43`
- **Signature:** `pub fn brief(ctx: &Ctx, budget: Option<usize>) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-brief]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
