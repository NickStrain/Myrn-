---
id: explain-call
type: summary
status: unverified
title: explain::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/explain.rs#tools::explain::call
anchors:
- sym:crates/myrn-mcp/src/tools/explain.rs#tools::explain::call@81bb77ca274a3990
---
# explain::call

Reads the required "symbol" argument with str_arg, calls engine.explain on it through a mutable Engine reference, and returns the card rendered to a String; errors from the argument read or from engine.explain are propagated with ?.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/explain.rs:20-23`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-explain-rs]]
- **Calls:** [[engine-explain]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
