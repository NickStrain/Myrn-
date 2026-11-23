---
id: connect-choose
type: summary
status: unverified
title: connect::choose
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::choose
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::choose@57213acd541ef83b
map_version: '2'
---
# connect::choose

Returns the adapters to install: the named ones from --agent, or the detected ones when no names are given, with errors when the choice is ambiguous or empty.

## Behaviour

- agents=["claude","x"] -> error `unknown agent` listing claude and codex
- no agents given, none detected -> error saying to use --agent to install anyway
- several detected and all=false -> error listing names joined with and; all=true -> returns all of them

## Terms

agent detection, adapter selection, detected agents, ambiguous agent, no agent found, claude code, codex

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:29-53`
- **Signature:** `fn choose(args: &Args) -> Result<Vec<Box<dyn Adapter>>>`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Calls:** [[all]], [[by-name]]
- **Called by:** [[connect-run]]
