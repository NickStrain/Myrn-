---
id: dir-crates-myrn-adapters
type: summary
status: unverified
title: crates/myrn-adapters/
source: myrn summarize
about: module:crates/myrn-adapters
anchors:
- module:crates/myrn-adapters@bee92d022f7945f3
---
# crates/myrn-adapters/

Per-agent integration layer for myrn: adapter.rs defines the Adapter trait (detect, install, uninstall, inspect, payload translation, output formatting) plus shared JSON and hook-editing helpers, and lib.rs exposes the registry (all, by_name) over the claude_code and codex implementations. pull_only.rs manages the marked instruction block that tells agents to call the MCP `context` tool, and detect.rs locates installed agents and PATH binaries.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-adapters-src]]
