---
id: dir-crates-myrn
type: summary
status: unverified
title: crates/myrn/
source: myrn summarize
about: module:crates/myrn
anchors:
- module:crates/myrn@a0239ef7d89ecb2b
---
# crates/myrn/

The `myrn` CLI binary crate with a clap-based dispatcher in src/main.rs that builds a cmd::Ctx from global flags and routes each subcommand (including `mcp`, which runs the MCP server) to a handler in src/cmd/; most handlers build an engine from Ctx, call one engine method, and print via ctx.emit, while hook.rs is the never-fail entry for agent hook scripts and diff/forget/gc/publish/trace/verify are stubs. tests/connect.rs drives the real binary end to end against a temp MYRN_HOME and temp git repo, using the setup/Env/hook/settings/text helpers, to cover init, connect consent and hooks, doctor, disconnect, and episode log and graph output.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-src]], [[dir-crates-myrn-tests]]
