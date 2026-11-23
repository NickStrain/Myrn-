---
id: tools-str-arg
type: summary
status: unverified
title: tools::str_arg
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/mod.rs#tools::str_arg
anchors:
- sym:crates/myrn-mcp/src/tools/mod.rs#tools::str_arg@1d22bec0d083a831
map_version: '2'
---
# tools::str_arg

Returns the named argument as a string slice, rejecting missing, non-string, or blank values with an error. The value is not trimmed.

## Behaviour

- {"q": " foo "} with key q -> Ok(" foo "), whitespace kept
- {"q": " "}, a non-string value, or no such key -> Err("missing required argument `q`")

## Terms

required argument, string argument, blank check, argument validation, missing parameter

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs:102-107`
- **Signature:** `pub(crate) fn str_arg<'a>(args: &'a Value, key: &str) -> Result<&'a str>`
- **In:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Called by:** [[confirm-call]], [[context-call]], [[explain-call]], [[history-call]], [[impact-call]], [[recall-call]], [[supersede-call]]
