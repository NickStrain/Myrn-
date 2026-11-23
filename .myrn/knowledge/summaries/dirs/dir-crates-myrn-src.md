---
id: dir-crates-myrn-src
type: summary
status: unverified
title: crates/myrn/src/
source: myrn summarize
about: module:crates/myrn/src
anchors:
- module:crates/myrn/src@00e5a5f206666fa1
---
# crates/myrn/src/

Source for the `myrn` binary: `main.rs` defines the clap CLI (global flags, every subcommand, embeddings and debug subcommands), parses arguments, builds `cmd::Ctx`, and dispatches to the matching handler, printing errors to stderr as `myrn: ...` with a failure exit code. `cmd/` holds one thin handler module per subcommand, declared in `cmd/mod.rs` alongside the shared `Ctx` that opens the engine and emits output; each handler parses its arguments, calls one engine method, and renders text or JSON, while the real logic lives in the engine and sibling crates.

## Connections

- **In:** [[dir-crates-myrn]]
- **Directories:** [[dir-crates-myrn-src-cmd]]
- **Files:** [[file-crates-myrn-src-main-rs]]
