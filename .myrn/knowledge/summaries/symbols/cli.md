---
id: cli
type: summary
status: unverified
title: Cli
source: myrn summarize
about: sym:bench/src/main.rs#Cli
anchors:
- sym:bench/src/main.rs#Cli@8a1181aaa55a65d4
map_version: '2'
---
# Cli

Holds the global options shared by all subcommands: the bench directory, an optional checkouts directory, and the chosen subcommand.

## Behaviour

- bench_dir unset -> defaults to the crate's CARGO_MANIFEST_DIR
- checkouts unset -> real_main falls back to <bench-dir>/repos/checkouts
- global flags are accepted before or after the subcommand

## Terms

command line, arguments, flags, options, bench directory, clap parser, global options

## Connections

- **Location:** `bench/src/main.rs:26-35`
- **Signature:** `struct Cli`
- **In:** [[file-bench-src-main-rs]]
