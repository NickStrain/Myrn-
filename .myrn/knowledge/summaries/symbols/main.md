---
id: main
type: summary
status: unverified
title: main
source: myrn summarize
about: sym:bench/src/main.rs#main
anchors:
- sym:bench/src/main.rs#main@caa4973d35f4372c
---
# main

Calls real_main, prints the error chain to stderr prefixed with myrn-bench: on failure, and maps any error to ExitCode::FAILURE.

## Connections

- **Location:** `bench/src/main.rs:98-106`
- **Signature:** `fn main() -> ExitCode`
- **In:** [[file-bench-src-main-rs]]
- **Calls:** [[real-main]]
