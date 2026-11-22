---
id: recall-call
type: summary
status: unverified
title: recall::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/recall.rs#tools::recall::call
anchors:
- sym:crates/myrn-mcp/src/tools/recall.rs#tools::recall::call@b6beee73394f3e39
---
# recall::call

Reads `error` (required string) and `budget` (defaults to 1500 tokens), calls `engine.recall_text`, and returns the matched text or the literal message "No recorded bug matches this error." when nothing matches. Only when text is returned does it log the matched ids via `engine.log_injection("mcp", ...)`, so an empty result has no logging side effect.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/recall.rs:21-30`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-recall-rs]]
- **Calls:** [[engine-log-injection]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
