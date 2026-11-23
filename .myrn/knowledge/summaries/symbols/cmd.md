---
id: cmd
type: summary
status: unverified
title: Cmd
source: myrn summarize
about: sym:bench/src/main.rs#Cmd
anchors:
- sym:bench/src/main.rs#Cmd@9300d505515fc949
map_version: '2'
---
# Cmd

Enumerates the subcommands (List, Fetch, Validate, Run, Retrieval, Report, Review), each with its own flags and help text.

## Behaviour

- Run --runs defaults to 3 runs per task and condition
- Run --conditions defaults to 'without,with' and accepts 'off' as well
- Validate and Run --task take comma-separated id prefixes; unset selects all tasks

## Terms

subcommand, commands, run mode, list tasks, validate tasks, fetch checkouts, retrieval check, review sample

## Connections

- **Location:** `bench/src/main.rs:38-130`
- **Signature:** `enum Cmd`
- **In:** [[file-bench-src-main-rs]]
