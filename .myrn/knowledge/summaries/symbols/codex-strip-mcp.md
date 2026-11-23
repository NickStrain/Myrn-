---
id: codex-strip-mcp
type: summary
status: unverified
title: codex::strip_mcp
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::strip_mcp
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::strip_mcp@2ad9524e18858aa3
map_version: '2'
---
# codex::strip_mcp

Deletes the managed block between the start and end markers from TOML text and returns the rest.

## Behaviour

- markers missing, or end before start -> input returned unchanged
- block at the start of the file -> the text after the block, with no leading blank line
- block in the middle -> text before and after joined by one newline

## Terms

delete block, marker removal, managed section removal

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:62-80`
- **Signature:** `fn strip_mcp(s: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-write-mcp]], [[codex-uninstall]]
