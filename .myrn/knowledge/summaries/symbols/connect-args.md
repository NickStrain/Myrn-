---
id: connect-args
type: summary
status: unverified
title: connect::Args
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::Args
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::Args@e61528aa687fc6ab
map_version: '2'
---
# connect::Args

Holds the parsed CLI flags for `myrn connect`: which agents to install, whether to install to all detected agents, whether to skip hooks, and whether to skip the consent prompt.

## Behaviour

- all=false, agents empty, one agent detected -> that agent is chosen by choose
- agents=["codex"] -> choose uses codex even if it was not detected
- pull_only=true -> run uses Mode::PullOnly and does not ask for hook consent

## Terms

flags, cli options, agent selection, pull only, skip prompt, install all

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:12-17`
- **Signature:** `pub struct Args`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
