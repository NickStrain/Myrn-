---
id: cli-main
type: summary
status: unverified
title: Cli
source: myrn summarize
about: sym:crates/myrn/src/main.rs#Cli
anchors:
- sym:crates/myrn/src/main.rs#Cli@a1f1d8e3a53a4a24
map_version: '2'
---
# Cli

Top-level clap parser holding the global `--data-dir` (hidden, env `MYRN_HOME`) and `--json` flags plus the selected subcommand.

## Behaviour

- `--data-dir` unset and no `MYRN_HOME` -> the platform per-user data directory is used (per the doc comment)
- `--json` is global, so it can follow any subcommand and switches output to JSON
- `--data-dir` is hidden from `--help` but still accepted

## Terms

global flags, command line options, data directory, json output, argument parsing, root parser

## Connections

- **Location:** `crates/myrn/src/main.rs:18-29`
- **Signature:** `struct Cli`
- **In:** [[file-crates-myrn-src-main-rs]]
