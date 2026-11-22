---
id: tools-str-arg
type: summary
status: unverified
title: tools::str_arg
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/mod.rs#tools::str_arg
anchors:
- sym:crates/myrn-mcp/src/tools/mod.rs#tools::str_arg@1d22bec0d083a831
---
# tools::str_arg

Returns args[key] as a &str that borrows from args. It errors with 'missing required argument' if the key is absent, not a string, or blank after trimming whitespace.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs:50-55`
- **Signature:** `pub(crate) fn str_arg<'a>(args: &'a Value, key: &str) -> Result<&'a str>`
- **In:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Called by:** [[context-call]], [[explain-call]], [[history-call]], [[impact-call]], [[recall-call]]
