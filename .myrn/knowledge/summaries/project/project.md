---
id: project
type: summary
status: unverified
title: Project
source: myrn summarize
about: module:.
anchors:
- module:.@e7c2f88d2533c4db
---
# Project

Myrn is a local Rust tool that gives AI coding agents persistent project memory. It parses a codebase with Tree-sitter into a graph of symbols, calls, imports, tests and Git history, stores it in one SQLite file, and returns task-sized context within a token budget. The crates/ workspace splits this into myrn-model (pure domain types), myrn-store (storage), myrn-engine (the core, which adapters, the MCP server and the CLI reach only through its api.rs), myrn-adapters for agent integrations, myrn-embed for optional embeddings, myrn-mcp for the JSON-RPC server, and the myrn binary, whose subcommands dispatch to the engine. dist/ packages each release for Homebrew, Scoop, winget and npm, where the npm wrapper downloads and checksum-verifies the native binary at postinstall. bench/ is a Rust CLI (myrn-bench) that measures whether myrn helps Claude Code across A/B conditions and checks retrieval quality, and tests/fixtures holds sample repositories for exercising the code map.

## Connections

- **Directories:** [[dir-bench]], [[dir-crates]], [[dir-dist]], [[dir-tests]]
