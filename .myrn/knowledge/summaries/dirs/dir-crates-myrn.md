---
id: dir-crates-myrn
type: summary
status: unverified
title: crates/myrn/
source: myrn summarize
about: module:crates/myrn
anchors:
- module:crates/myrn@7d037225327fd3b3
---
# crates/myrn/

crates/myrn/ is the command-line binary crate for myrn. src/main.rs defines the clap CLI, parses arguments, builds a cmd::Ctx and dispatches to one thin handler module per subcommand under src/cmd/, which call engine methods and render text or JSON, with the real logic living in the engine and sibling crates. tests/connect.rs runs the built binary end to end against a temp git repo covering init, connect, hooks, doctor, Codex setup and disconnect.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-src]], [[dir-crates-myrn-tests]]
