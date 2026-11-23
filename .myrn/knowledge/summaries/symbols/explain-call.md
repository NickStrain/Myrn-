---
id: explain-call
type: summary
status: unverified
title: explain::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/explain.rs#tools::explain::call
anchors:
- sym:crates/myrn-mcp/src/tools/explain.rs#tools::explain::call@81bb77ca274a3990
map_version: '2'
---
# explain::call

Reads the `symbol` argument, asks the engine to explain it, and returns the resulting card rendered as a string.

## Behaviour

- args {"symbol": "a::b"} -> engine.explain("a::b") result rendered via e.render() as Ok(String)
- missing or non-string "symbol" -> error from str_arg propagated with ?
- engine.explain error -> propagated as Err; takes &mut Engine, so the lookup may change engine state

## Terms

explain handler, tool invocation, symbol lookup, card rendering, dispatch, engine call

## Connections

- **Location:** `crates/myrn-mcp/src/tools/explain.rs:20-23`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-explain-rs]]
- **Calls:** [[engine-explain]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
