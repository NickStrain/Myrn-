---
id: project
type: summary
status: unverified
title: Project
source: myrn summarize
about: module:.
anchors:
- module:.@7cce9b0355ddda1d
---
# Project

Myrn is a local, persistent project-memory tool for AI coding agents: it indexes a codebase into a graph of symbols, calls, imports, tests and Git history (parsed with Tree-sitter for Rust, Python, JavaScript and TypeScript) so an agent can fetch task-sized context instead of re-reading files. The Rust workspace under crates/ separates concerns: myrn-model holds pure domain types, myrn-store persists to SQLite plus Markdown notes and JSONL episode logs, myrn-engine exposes the Engine API in api.rs, and the myrn CLI and myrn-mcp stdio JSON-RPC server both call into it. myrn-adapters wires per-agent hooks and config for Claude Code and Codex, and myrn-embed is an optional long-lived embedding helper. The bench/ directory builds the myrn-bench binary, which runs Claude Code headless on pinned repo clones with and without myrn and reports paired results with a Gate 1 sign-test verdict, while tests/fixtures/ holds a multi-language token-refresh fixture repo used for testing.

## Connections

- **Directories:** [[dir-bench]], [[dir-crates]], [[dir-tests]]
