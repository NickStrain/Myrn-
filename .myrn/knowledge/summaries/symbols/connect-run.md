---
id: connect-run
type: summary
status: unverified
title: connect::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::run
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::run@6c5901b5deaa8834
map_version: '2'
---
# connect::run

Runs the connect flow: asks for consent when hooks are requested and not yet granted, installs each chosen adapter, merges the created-file list into the agent's state, and saves state.

## Behaviour

- hooks mode, consent not yet given, user declines -> prints nothing installed, returns FAILURE, writes nothing
- consent already given -> no prompt, installs directly and keeps existing state.created entries
- an adapter install error stops the loop with ? before save_state, so consent and state from this run are not persisted

## Terms

connect, install hooks, consent, setup, session recording, agent state, pull only, mcp tools

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:78-140`
- **Signature:** `pub fn run(ctx: &Ctx, args: Args) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Calls:** [[mode-as-str]], [[engine-open]], [[engine-root]], [[engine-state]], [[engine-save-state]], [[connect-install-ctx]], [[connect-choose]], [[connect-ask]], [[connect-now]]
- **Called by:** [[main-main]]
