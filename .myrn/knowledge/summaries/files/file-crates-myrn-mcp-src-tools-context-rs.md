---
id: file-crates-myrn-mcp-src-tools-context-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/context.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/tools/context.rs
anchors:
- file:crates/myrn-mcp/src/tools/context.rs@529a61a4a2aba822
---
# crates/myrn-mcp/src/tools/context.rs

Defines the MCP `context` tool for the myrn-mcp server. `spec()` returns the tool's JSON schema (`task` required, `budget` optional), and `call()` asks the engine for a ranked context pack for the task, logs the pack's items as an injection under source `mcp`, and returns the pack text.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/context.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[context-spec]], [[context-call]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
