---
id: dir-crates-myrn-adapters-src
type: summary
status: unverified
title: crates/myrn-adapters/src/
source: myrn summarize
about: module:crates/myrn-adapters/src
anchors:
- module:crates/myrn-adapters/src@99d01cddeaff079e
---
# crates/myrn-adapters/src/

Per-agent integration code for myrn: adapter.rs defines the Adapter trait (detect, install, uninstall, inspect, payload translation, output formatting) and shared JSON/hook-editing helpers, and lib.rs exposes the registry (all, by_name) over the claude_code and codex implementations. pull_only.rs manages the shared marked instruction block that directs agents to call the MCP `context` tool, and detect.rs locates installed agents and PATH binaries.

## Connections

- **In:** [[dir-crates-myrn-adapters]]
- **Files:** [[file-crates-myrn-adapters-src-adapter-rs]], [[file-crates-myrn-adapters-src-claude-code-rs]], [[file-crates-myrn-adapters-src-codex-rs]], [[file-crates-myrn-adapters-src-detect-rs]], [[file-crates-myrn-adapters-src-lib-rs]], [[file-crates-myrn-adapters-src-pull-only-rs]]
