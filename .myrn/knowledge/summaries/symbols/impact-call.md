---
id: impact-call
type: summary
status: unverified
title: impact::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/impact.rs#tools::impact::call
anchors:
- sym:crates/myrn-mcp/src/tools/impact.rs#tools::impact::call@4606686019b89d9f
---
# impact::call

Reads the `symbol` argument, calls `engine.impact` on the mutable Engine, and returns the result rendered with a limit of 15 entries as a String. Errors propagate from missing or invalid arguments or from the engine call.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/impact.rs:20-23`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-impact-rs]]
- **Calls:** [[engine-impact]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
