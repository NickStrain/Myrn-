---
id: hook-run
type: summary
status: unverified
title: hook::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/hook.rs#cmd::hook::run
anchors:
- sym:crates/myrn/src/cmd/hook.rs#cmd::hook::run@3bdd5f6f241ff22b
map_version: '2'
---
# hook::run

Calls handle and prints its non-empty output to stdout. Any error from handle goes to stderr and the process still exits with SUCCESS.

## Behaviour

- handle returns non-empty text -> printed with a trailing newline
- handle returns empty string -> nothing printed
- handle returns Err -> "myrn hook: <error chain>" on stderr, exit code 0

## Terms

exit code, error swallowing, stdout print, fail open, silent failure, never block agent

## Connections

- **Location:** `crates/myrn/src/cmd/hook.rs:22-33`
- **Signature:** `pub fn run(ctx: &Ctx, agent: &str, event: &str) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-hook-rs]]
- **Calls:** [[hook-handle]]
- **Called by:** [[main-main]]
