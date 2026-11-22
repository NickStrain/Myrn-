---
id: query-context
type: summary
status: unverified
title: query::context
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::context
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::context@cdeb13738b43c85d
---
# query::context

Calls engine.context(task, budget) with an optional token budget and emits the result. If the returned text is empty, the text output says no relevant code-graph context was found; otherwise it prints the text followed by the token count and item count.

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:27-37`
- **Signature:** `pub fn context(ctx: &Ctx, task: &str, budget: Option<usize>) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-context]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
