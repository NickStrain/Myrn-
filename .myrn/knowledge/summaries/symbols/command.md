---
id: command
type: summary
status: unverified
title: Command
source: myrn summarize
about: sym:crates/myrn/src/main.rs#Command
anchors:
- sym:crates/myrn/src/main.rs#Command@f9d83b5ba611137a
map_version: '2'
---
# Command

Enum of every `myrn` subcommand with its arguments and flags, each variant's doc comment serving as its help text.

## Behaviour

- `myrn log -n 5 --type bug` -> limit 5, ntype Some("bug"), other fields at defaults
- `myrn context fix the bug` -> task is the word list (main joins it with spaces); `--budget` optional
- `Hook` and `EmbedHelper` are hidden from help; `Gc` defaults `older_than` to 30 days

## Terms

subcommands, cli verbs, command list, flags, arguments, help text

## Connections

- **Location:** `crates/myrn/src/main.rs:32-206`
- **Signature:** `enum Command`
- **In:** [[file-crates-myrn-src-main-rs]]
