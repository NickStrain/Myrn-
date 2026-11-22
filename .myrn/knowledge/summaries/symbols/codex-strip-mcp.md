---
id: codex-strip-mcp
type: summary
status: unverified
title: codex::strip_mcp
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::strip_mcp
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::strip_mcp@2ad9524e18858aa3
---
# codex::strip_mcp

Removes the managed block, markers included, from TOML text and returns the rest. If the markers are missing or out of order it returns the input unchanged.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:62-80`
- **Signature:** `fn strip_mcp(s: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-write-mcp]], [[codex-uninstall]]
