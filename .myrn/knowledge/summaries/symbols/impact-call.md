---
id: impact-call
type: summary
status: unverified
title: impact::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/impact.rs#tools::impact::call
anchors:
- sym:crates/myrn-mcp/src/tools/impact.rs#tools::impact::call@4606686019b89d9f
map_version: '2'
---
# impact::call

Reads the 'symbol' argument, runs engine.impact on it, and returns the result rendered as a string with render(15) applied to each entry.

## Behaviour

- args missing a string 'symbol' -> Err from str_arg, engine is not called
- engine.impact returns Err -> that error is propagated with ?
- valid symbol -> rendered impact report as Ok(String), with render(15) passed to the inner render

## Terms

impact report, dependents lookup, callers and tests, impact analysis, render impact, reverse dependencies

## Connections

- **Location:** `crates/myrn-mcp/src/tools/impact.rs:20-23`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-impact-rs]]
- **Calls:** [[engine-impact]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
