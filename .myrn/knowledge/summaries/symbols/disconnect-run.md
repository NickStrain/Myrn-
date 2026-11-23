---
id: disconnect-run
type: summary
status: unverified
title: disconnect::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/disconnect.rs#cmd::disconnect::run
anchors:
- sym:crates/myrn/src/cmd/disconnect.rs#cmd::disconnect::run@808e143771f46985
map_version: '2'
---
# disconnect::run

Uninstalls the named agents, or all agents in state if none are given, through their adapters, prints each change, removes them from state, and saves state.

## Behaviour

- no agents given and state has none -> prints 'No agents are connected.' and returns success without saving
- unknown name -> prints 'unknown agent `name`', skips it, and leaves its state entry untouched
- an adapter uninstall error -> returns Err before save_state, so no state changes are written

## Terms

disconnect, uninstall, remove hooks, unregister MCP, agent state, capture consent, consent reset, instruction lines

## Connections

- **Location:** `crates/myrn/src/cmd/disconnect.rs:9-49`
- **Signature:** `pub fn run(ctx: &Ctx, agents: Vec<String>) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-disconnect-rs]]
- **Calls:** [[by-name]], [[engine-open]], [[engine-root]], [[engine-state]], [[engine-save-state]], [[connect-install-ctx]]
- **Called by:** [[main-main]]
