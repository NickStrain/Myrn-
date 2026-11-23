---
id: query-context
type: summary
status: unverified
title: query::context
source: myrn summarize
about: sym:crates/myrn/src/cmd/query.rs#cmd::query::context
anchors:
- sym:crates/myrn/src/cmd/query.rs#cmd::query::context@cdeb13738b43c85d
map_version: '2'
---
# query::context

Builds a context pack for a task string and emits its text with a token and item count, or a fixed message when nothing relevant is found.

## Behaviour

- task: &str, budget: None -> engine.context(task, None), uses the engine's default budget
- p.text empty -> prints fixed message 'Nothing in the code graph is clearly relevant; an agent would get no injected context.'
- p.text non-empty -> prints text, then '[N tokens, M items]' with p.tokens and p.items.len()

## Terms

injected context, prompt context, relevant code, token budget, task context, agent context

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs:27-37`
- **Signature:** `pub fn context(ctx: &Ctx, task: &str, budget: Option<usize>) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-query-rs]]
- **Calls:** [[engine-context]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
