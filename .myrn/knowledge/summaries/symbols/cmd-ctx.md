---
id: cmd-ctx
type: summary
status: unverified
title: cmd::Ctx
source: myrn summarize
about: sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx
anchors:
- sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx@7eac61cc6ee6510e
map_version: '2'
---
# cmd::Ctx

Carries the global options for one invocation: an optional data directory and the JSON output flag.

## Behaviour

- data_dir None -> passed through as None to Engine::open, so the engine picks its default location
- json true -> Ctx::emit prints pretty JSON; json false -> prints the text closure output
- Fields are plain pub data; no validation happens in this struct

## Terms

options, global flags, context, settings, data directory, output mode, json flag

## Connections

- **Location:** `crates/myrn/src/cmd/mod.rs:32-35`
- **Signature:** `pub struct Ctx`
- **In:** [[file-crates-myrn-src-cmd-mod-rs]]
