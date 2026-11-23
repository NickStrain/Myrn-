---
id: context-call
type: summary
status: unverified
title: context::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/context.rs#tools::context::call
anchors:
- sym:crates/myrn-mcp/src/tools/context.rs#tools::context::call@aa0358c1c23f370a
map_version: '2'
---
# context::call

Reads `task` and an optional `budget` from the arguments, asks the engine for a context pack, logs the pack items when non-empty, and returns the pack text or a fixed no-match message.

## Behaviour

- {"task":"parse config"} -> budget None, so the engine's own default applies; returns pack.text
- {"task":"x","budget":800} -> budget Some(800); a non-integer or negative budget is treated as absent
- missing or non-string `task` -> error from str_arg propagates via ?; the engine is not called

## Terms

context pack, ranked context, relevant symbols, token budget, context request, injection log, no match, empty result

## Connections

- **Location:** `crates/myrn-mcp/src/tools/context.rs:21-33`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-context-rs]]
- **Calls:** [[engine-context]], [[engine-log-injection]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
