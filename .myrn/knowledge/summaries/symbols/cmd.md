---
id: cmd
type: summary
status: unverified
title: Cmd
source: myrn summarize
about: sym:bench/src/main.rs#Cmd
anchors:
- sym:bench/src/main.rs#Cmd@05956d8d8b4a58d4
---
# Cmd

Subcommands: List, Fetch (clones missing repos over the network), Validate (checks each task without an agent), Run (agent sessions; --runs defaults to 3, --conditions to without,with, --permission-mode to bypassPermissions, supports --dry-run), Report (re-renders report.md, optionally rescoring first), and Review (writes a sampled review.md or scores a marked one, defaults 30 samples and seed 1).

## Connections

- **Location:** `bench/src/main.rs:37-96`
- **Signature:** `enum Cmd`
- **In:** [[file-bench-src-main-rs]]
