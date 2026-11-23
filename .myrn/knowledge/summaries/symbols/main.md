---
id: main
type: summary
status: unverified
title: main
source: myrn summarize
about: sym:bench/src/main.rs#main
anchors:
- sym:bench/src/main.rs#main@caa4973d35f4372c
map_version: '2'
---
# main

Calls real_main and converts its result into the process exit code, printing any error to stderr first.

## Behaviour

- real_main returns Ok(code) -> that ExitCode is returned unchanged
- real_main returns Err -> prints 'myrn-bench: ' followed by the full error chain to stderr and exits with FAILURE
- no error -> exit status comes only from the subcommand

## Terms

entry point, exit code, error output, failure status, startup

## Connections

- **Location:** `bench/src/main.rs:132-140`
- **Signature:** `fn main() -> ExitCode`
- **In:** [[file-bench-src-main-rs]]
- **Calls:** [[real-main]]
