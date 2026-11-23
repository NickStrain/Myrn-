---
id: seed-code-tokens
type: summary
status: unverified
title: seed::code_tokens
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::code_tokens
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::code_tokens@e790f4544ca71647
map_version: '2'
---
# seed::code_tokens

Splits task text on whitespace and punctuation, keeping tokens that look like code: snake_case, CamelCase, paths, a::b, dotted names and file:line.

## Behaviour

- refresh_token() -> refresh_token (brackets split off)
- src/auth/session.rs:42 -> kept as one token
- plain capitalised word like Fix -> dropped (no inner capital, underscore, path or dot)

## Terms

code token, identifier, snake case, camel case, file line, qualified name, path token

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:168-187`
- **Signature:** `fn code_tokens(task: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[seed-task-words]], [[seed-seeds]]
- **Tests:** `code_like_tokens`
